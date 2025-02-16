use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;
use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};
use office::{Excel, Range, DataType};
use crate::utils::{usize_float_multiplication, calculate_distances};

/// Represents a distance matrix for a TSP instance.
pub struct DistanceMatrix {
    distances: Vec<Vec<f64>>,
}

impl DistanceMatrix {
    pub fn new(num_cities: usize) -> Self {
        Self {
            distances: vec![vec![0.0; num_cities]; num_cities],
        }
    }

    /// Build a DistanceMatrix from an existing 2D vector.
    pub fn from_vec(distances: Vec<Vec<f64>>) -> Self {
        Self { distances }
    }

    pub fn get_distance(&self, from: usize, to: usize) -> f64 {
        self.distances[from][to]
    }

    pub fn set_distance(&mut self, from: usize, to: usize, distance: f64) {
        self.distances[from][to] = distance;
        self.distances[to][from] = distance; // Assuming symmetric TSP
    }

    pub fn len(&self) -> usize {
        self.distances.len()
    }

    pub fn as_vec(&self) -> &Vec<Vec<f64>> {
        &self.distances
    }
}

/// Represents a single ant in the ACO algorithm.
pub struct Ant {
    pub current_city: usize,
    pub distance_traveled: f64,
    pub visited_cities: Vec<usize>,
    pub start_city: usize,
    pub path_taken: Vec<usize>,
    pub alpha: f64,
    pub beta: f64,
}

impl Ant {
    pub fn new(num_cities: usize, alpha: f64, beta: f64) -> Self {
        let current_city = rand::thread_rng().gen_range(0..num_cities);
        Self {
            current_city,
            distance_traveled: 0.0,
            visited_cities: Vec::new(),
            start_city: current_city,
            path_taken: Vec::new(),
            alpha,
            beta,
        }
    }

    pub fn generate_path(&mut self, model: &AcoModel) {
        self.visited_cities.clear();
        let mut result_path: Vec<usize> = Vec::new();
        self.visited_cities.push(self.current_city);
        for _ in 0..model.cities.len() - 1 {
            result_path.push(self.current_city);
            let next_city = self.pick_move(model);
            self.distance_traveled += model.distances.get_distance(self.current_city, next_city);
            self.current_city = next_city;
            self.visited_cities.push(self.current_city);
        }
        self.distance_traveled += model.distances.get_distance(self.current_city, self.start_city);
        result_path.push(self.start_city);
        self.path_taken = result_path;
    }

    pub fn pick_move(&self, model: &AcoModel) -> usize {
        let current_city = self.current_city;
        let mut rng = rand::thread_rng();
        let mut row_probabilities: Vec<f64> = Vec::new();

        // Calculate probabilities for moving to each unvisited city.
        for (index, &pheromone) in model.pheromones[current_city].iter().enumerate() {
            if !self.visited_cities.contains(&index) {
                let dist = 1.0 / model.distances.get_distance(current_city, index);
                let probability = pheromone.powf(self.alpha) * dist.powf(self.beta);
                row_probabilities.push(probability);
            } else {
                row_probabilities.push(0.0);
            }
        }
        let total_probability: f64 = row_probabilities.iter().sum();
        if total_probability == 0.0 {
            return self.start_city;
        }
        let dist = WeightedIndex::new(&row_probabilities)
            .expect("Issue with initiating probability for next move");
        model.cities[dist.sample(&mut rng)]
    }
}

/// Represents the overall ACO model.
pub struct AcoModel {
    pub cities: Vec<usize>,
    pub distances: DistanceMatrix,
    pub best_distance: f64,
    pub best_path: Vec<usize>,
    pub pheromones: Vec<Vec<f64>>,
    pub pheromone_value: f64,
    pub decay: f64,
    pub number_of_iterations: usize,
    pub ant_count: usize,
    pub init_alpha: f64,
    pub init_beta: f64,
    pub final_alpha: f64,
    pub final_beta: f64,
    pub alpha_scaling: f64,
    pub beta_scaling: f64,
    pub city_names: HashMap<usize, String>,
    pub rank_limit: u32,
}

impl AcoModel {
    fn update_pheromones(&mut self, ants: &mut Vec<Ant>, _average_distance: f64) {
        // Evaporation step.
        for row in self.pheromones.iter_mut() {
            for pheromone in row.iter_mut() {
                *pheromone *= self.decay;
            }
        }
        ants.sort_by(|a, b| a.distance_traveled.partial_cmp(&b.distance_traveled).unwrap());
        // Deposit step for the top-ranked ants.
        for (rank, ant) in ants.iter().enumerate() {
            if (rank as u32) <= self.rank_limit {
                let weight = ((self.rank_limit - (rank as u32)) as f64) / (self.rank_limit as f64);
                for window in ant.path_taken.windows(2) {
                    if let [from, to] = window {
                        if from != to {
                            let pheromone_deposit =
                                (self.pheromone_value * weight) / self.distances.get_distance(*from, *to);
                            self.pheromones[*from][*to] += pheromone_deposit;
                            self.pheromones[*to][*from] += pheromone_deposit;
                        }
                    }
                }
            }
        }
    }

    fn print_results(&self) {
        if !self.city_names.is_empty() {
            let best_path_cities: Vec<String> = self.best_path
                .iter()
                .map(|city| self.city_names[city].clone())
                .collect();
            println!("Best path (indices): {:?}", self.best_path);
            println!("Best path (cities): {:?}", best_path_cities);
        } else {
            println!("Best path: {:?}", self.best_path);
        }
        println!("Best distance: {:.2}", self.best_distance);
    }

    fn calculate_average_distance(ants: &Vec<Ant>) -> f64 {
        let total_distance: f64 = ants.iter().map(|ant| ant.distance_traveled).sum();
        total_distance / (ants.len() as f64)
    }

    fn new(distances: Vec<Vec<f64>>, city_names: Option<HashMap<usize, String>>) -> Self {
        let num_cities = distances.len();
        let city_names = city_names.unwrap_or(HashMap::new());
        let cities: Vec<usize> = (0..num_cities).collect();
        let best_distance = f64::MAX;
        let best_path = vec![];
        let pheromone_value = 4.0;
        let pheromones = vec![vec![0.5; num_cities]; num_cities];
        let number_of_iterations = 55;
        let decay = 0.5;
        let ant_count = 5555;
        let init_alpha = 1.1;
        let init_beta = 1.0;
        let final_alpha = init_alpha;
        let final_beta = init_beta;
        let alpha_scaling = 0.85;
        let beta_scaling = 1.3;
        let rank_limit = 10;
        Self {
            cities,
            city_names,
            distances: DistanceMatrix::from_vec(distances),
            best_distance,
            best_path,
            pheromone_value,
            decay,
            pheromones,
            number_of_iterations,
            ant_count,
            init_alpha,
            init_beta,
            final_alpha,
            final_beta,
            alpha_scaling,
            beta_scaling,
            rank_limit,
        }
    }

    /// Create a new AcoModel from a text file containing a space-delimited distance matrix.
    pub fn new_from_file(file_path: &str) -> Result<AcoModel, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut num_cities = 0;
        let mut distances: Vec<Vec<f64>> = vec![];
        for line in reader.lines() {
            let line = line?;
            if line.starts_with("#") || line.trim().is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if num_cities == 0 {
                num_cities = parts.len();
            }
            let row: Vec<f64> = parts.iter().map(|&x| x.parse().unwrap()).collect();
            distances.push(row);
        }
        if distances.len() != num_cities {
            return Err("The number of rows does not match the number of cities".into());
        }
        Ok(AcoModel::new(distances, None))
    }

    /// Create a new AcoModel from an Excel file.
    pub fn new_from_excel(file_path: &str, sheet: Option<&str>) -> Result<AcoModel, Box<dyn Error>> {
        let sheet_name = sheet.unwrap_or("Sheet1").to_string();
        let mut workbook = Excel::open(file_path).expect("Cannot open Excel file");
        let mut cities: HashMap<String, Vec<f64>> = HashMap::new();
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            for row in range.rows() {
                if row.len() >= 3 {
                    let city_name = match &row[0] {
                        DataType::String(name) => name.clone(),
                        _ => panic!("Expected a string for city name"),
                    };
                    let longitude = match &row[1] {
                        DataType::Float(val) => *val,
                        DataType::String(val) => val.parse::<f64>().unwrap_or_else(|_| {
                            panic!("Expected a float value for longitude found String({})", val)
                        }),
                        _ => panic!("Expected a float value for longitude found {:?}", &row[1]),
                    };
                    let latitude = match &row[2] {
                        DataType::Float(val) => *val,
                        DataType::String(val) => val.parse::<f64>().unwrap_or_else(|_| {
                            panic!("Expected a float value for latitude found String({})", val)
                        }),
                        _ => panic!("Expected a float value for latitude found {:?}", &row[2]),
                    };
                    cities.insert(city_name, vec![longitude, latitude]);
                } else {
                    panic!("Each row must have at least 3 columns");
                }
            }
        } else {
            panic!("Cannot find the specified worksheet");
        }
        drop(workbook);
        let city_indices: HashMap<usize, String> = cities
            .keys()
            .enumerate()
            .map(|(i, name)| (i, name.clone()))
            .collect();
        let coordinates: Vec<Vec<f64>> = cities.values().cloned().collect();
        let num_cities = cities.len();
        let mut distances = vec![vec![0.0; num_cities]; num_cities];
        for i in 0..num_cities {
            for j in 0..num_cities {
                if i == j {
                    distances[i][j] = 0.0;
                } else {
                    distances[i][j] = calculate_distances(
                        coordinates[i][1],
                        coordinates[j][1],
                        coordinates[i][0],
                        coordinates[j][0],
                    );
                }
            }
        }
        Ok(AcoModel::new(distances, Some(city_indices)))
    }

    /// Create a new AcoModel from a TSPLIB .tsp file.
    ///
    /// This method now supports both files with a NODE_COORD_SECTION (coordinates) and those
    /// with an EDGE_WEIGHT_SECTION (explicit full matrix). For your file (bays29), the
    /// EDGE_WEIGHT_SECTION is parsed as a full distance matrix.
    pub fn new_from_tsp(file_path: &str) -> Result<AcoModel, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut dimension: Option<usize> = None;
        let mut lines = Vec::new();
        for line in reader.lines() {
            lines.push(line?);
        }
        // Get the dimension from the header.
        for line in &lines {
            if line.starts_with("DIMENSION") {
                let parts: Vec<&str> = line
                    .split(|c: char| c == ':' || c.is_whitespace())
                    .filter(|s| !s.is_empty())
                    .collect();
                if parts.len() >= 2 {
                    dimension = parts[1].parse::<usize>().ok();
                }
            }
        }
        let dim = dimension.ok_or("Could not find DIMENSION in the .tsp file")?;
        // Check for EDGE_WEIGHT_SECTION.
        if lines.iter().any(|l| l.contains("EDGE_WEIGHT_SECTION")) {
            let mut matrix_numbers: Vec<f64> = Vec::new();
            let mut reading = false;
            for line in lines {
                if line.contains("EDGE_WEIGHT_SECTION") {
                    reading = true;
                    continue;
                }
                if reading {
                    let trimmed = line.trim();
                    if trimmed == "EOF" || trimmed.contains("DISPLAY_DATA_SECTION") {
                        break;
                    }
                    for num in trimmed.split_whitespace() {
                        matrix_numbers.push(num.parse::<f64>()?);
                    }
                }
            }
            if matrix_numbers.len() < dim * dim {
                return Err("Not enough numbers in EDGE_WEIGHT_SECTION".into());
            }
            let mut matrix = vec![vec![0.0; dim]; dim];
            for i in 0..dim {
                for j in 0..dim {
                    matrix[i][j] = matrix_numbers[i * dim + j];
                }
            }
            return Ok(AcoModel::new(matrix, None));
        }
        // Otherwise, check for NODE_COORD_SECTION to parse coordinates.
        if lines.iter().any(|l| l.contains("NODE_COORD_SECTION")) {
            let mut coordinates: Vec<(f64, f64)> = Vec::new();
            let mut reading = false;
            for line in lines {
                if line.contains("NODE_COORD_SECTION") {
                    reading = true;
                    continue;
                }
                if reading {
                    let trimmed = line.trim();
                    if trimmed == "EOF" {
                        break;
                    }
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 3 {
                        let x: f64 = parts[1].parse()?;
                        let y: f64 = parts[2].parse()?;
                        coordinates.push((x, y));
                    }
                }
            }
            if coordinates.len() != dim {
                return Err("Number of coordinates does not match DIMENSION".into());
            }
            let mut matrix = vec![vec![0.0; dim]; dim];
            for i in 0..dim {
                for j in 0..dim {
                    if i != j {
                        let dx = coordinates[i].0 - coordinates[j].0;
                        let dy = coordinates[i].1 - coordinates[j].1;
                        matrix[i][j] = (dx * dx + dy * dy).sqrt();
                    }
                }
            }
            return Ok(AcoModel::new(matrix, None));
        }
        Err("No valid section (EDGE_WEIGHT_SECTION or NODE_COORD_SECTION) found in .tsp file".into())
    }

    pub fn run_model(&mut self) {
        let mut iterations_without_improvement = 0;

        for iteration in 0..self.number_of_iterations {
            // Regenerate ants for this iteration.
            let mut ants: Vec<Ant> = (0..self.ant_count)
                .map(|_| Ant::new(self.cities.len(), self.final_alpha, self.final_beta))
                .collect();

            for ant in &mut ants {
                ant.generate_path(self);
            }

            let average_distance = AcoModel::calculate_average_distance(&ants);
            self.update_pheromones(&mut ants, average_distance);

            let mut improved = false;
            for ant in &ants {
                if ant.distance_traveled < self.best_distance {
                    println!(
                        "\n new best at {:?} \n beating previous best at {:?} \n on iteration {} \n with alpha of {} \n beta of {} \n",
                        ant.distance_traveled,
                        self.best_distance,
                        iteration,
                        self.final_alpha,
                        self.final_beta
                    );
                    self.best_distance = ant.distance_traveled;
                    self.best_path = ant.visited_cities.clone();
                    improved = true;
                }
            }

            if improved {
                iterations_without_improvement = 0;
            } else {
                iterations_without_improvement += 1;
                if iterations_without_improvement >= self.number_of_iterations / 10 {
                    self.final_alpha *= self.alpha_scaling;
                    self.final_beta *= self.beta_scaling;
                    iterations_without_improvement = 0;
                    println!("Alpha and beta adjusted");
                }
                if (self.final_alpha <= self.init_alpha / 2.0)
                    && (iterations_without_improvement >= self.number_of_iterations / 10 - 1)
                {
                    self.pheromones = vec![vec![0.5; self.distances.len()]; self.distances.len()];
                }
            }
        }
        self.print_results();
    }

    // Setter methods for tuning the model.
    pub fn set_number_of_iterations(&mut self, number_of_iterations: usize) {
        self.number_of_iterations = number_of_iterations;
    }

    pub fn set_ant_count(&mut self, ant_count: usize) {
        self.ant_count = ant_count;
    }

    pub fn set_pheromone_value(&mut self, pheromone_value: f64) {
        self.pheromone_value = pheromone_value;
    }

    pub fn set_init_alpha(&mut self, init_alpha: f64) {
        self.init_alpha = init_alpha;
        self.final_alpha = init_alpha;
    }

    pub fn set_init_beta(&mut self, init_beta: f64) {
        self.init_beta = init_beta;
        self.final_beta = init_beta;
    }

    pub fn set_decay(&mut self, decay: f64) {
        self.decay = decay;
    }

    pub fn set_alpha_beta_scaling(&mut self, alpha: f64, beta: f64) {
        self.alpha_scaling = alpha;
        self.beta_scaling = beta;
    }

    pub fn set_rank_limit(&mut self, rank_limit: u32) {
        self.rank_limit = rank_limit;
    }

    pub fn return_best_result(&self) -> f64 {
        self.best_distance
    }

    // Placeholder for future graph representation methods.
    // For example, you could later add a method like `pub fn export_graph(&self) -> Graph` here.
}
