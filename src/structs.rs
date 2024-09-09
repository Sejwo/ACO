use rand::distributions::{ Distribution, WeightedIndex };
use rand::Rng;
use std::{ error::Error, fs::File, io::BufRead, io::BufReader, collections::HashMap };
use office::{ Excel, Range, DataType };
use crate::utils::{ usize_float_multiplication, calculate_distances };

struct Ant {
    current_city: usize,
    distance_traveled: f64,
    visited_cities: Vec<usize>,
    start_city: usize,
    path_taken: Vec<usize>,
    alpha: f64,
    beta: f64,
}

impl Ant {
    fn new(num_cities: usize, alpha: f64, beta: f64) -> Self {
        let current_city = rand::thread_rng().gen_range(0..num_cities);
        let distance_traveled = 0.0;
        let visited_cities = vec![];
        let start_city = current_city;
        let path_taken = vec![];
        Self {
            current_city,
            distance_traveled,
            visited_cities,
            start_city,
            path_taken,
            alpha,
            beta,
        }
    }

    fn generate_path(&mut self, model: &AcoModel) {
        self.visited_cities.clear();
        let mut result_path: Vec<usize> = vec![];
        self.visited_cities.push(self.current_city);
        for _ in 0..model.cities.len() - 1 {
            result_path.push(self.current_city);
            let next_city = self.pick_move(model);
            self.distance_traveled += model.distances[self.current_city][next_city];
            self.current_city = next_city;
            self.visited_cities.push(self.current_city);
        }
        self.distance_traveled += model.distances[self.current_city][self.start_city];
        result_path.push(self.start_city);
        self.path_taken = result_path;
    }

    fn pick_move(&self, model: &AcoModel) -> usize {
        let current_city = self.current_city;
        let mut rng = rand::thread_rng();
        let mut row_probabilities: Vec<f64> = Vec::new();
        //

        // Calculate row probabilities considering only unvisited cities
        for (index, &pheromone) in model.pheromones[current_city].iter().enumerate() {
            if !self.visited_cities.contains(&index) {
                let dist = 1.0 / model.distances[current_city][index];
                let probability = pheromone.powf(self.alpha) * dist.powf(self.beta);
                row_probabilities.push(probability);
            } else {
                row_probabilities.push(0.0); // Set probability to 0 for visited cities
            }
        }
        let total_probability: f64 = row_probabilities.iter().sum();
        if total_probability == 0.0 {
            return self.start_city;
        }
        let dist = WeightedIndex::new(&row_probabilities).expect(
            "Issue with initiating probability for next move: "
        );
        let chosen = model.cities[dist.sample(&mut rng)];
        chosen
    }
}

pub struct AcoModel {
    cities: Vec<usize>,
    distances: Vec<Vec<f64>>,
    best_distance: f64,
    best_path: Vec<usize>,
    pheromones: Vec<Vec<f64>>,
    pheromone_value: f64,
    decay: f64,
    number_of_iterations: usize,
    ant_count: usize,
    init_alpha: f64,
    init_beta: f64,
    final_alpha: f64,
    final_beta: f64,
    alpha_scaling: f64,
    beta_scaling: f64,
    city_names: HashMap<usize, String>,
    rank_limit: u32,
}

impl AcoModel {
    fn update_pheromones(&mut self, ants: &mut Vec<Ant>, _average_distance: f64) {
        // Evaporation step
        for row in self.pheromones.iter_mut() {
            for pheromone in row.iter_mut() {
                *pheromone *= self.decay;
            }
        }
        ants.sort_by(|a, b| a.distance_traveled.partial_cmp(&b.distance_traveled).unwrap());

        // Deposit step, only for ants with distance less than the average
        for (rank, ant) in ants.iter().enumerate() {
            if (rank as u32) <= self.rank_limit {
                let weight = ((self.rank_limit - (rank as u32)) as f64) / (self.rank_limit as f64);
                for window in ant.path_taken.windows(2) {
                    if let [from, to] = window {
                        if from != to {
                            let pheromone_deposit =
                                (self.pheromone_value * weight) / self.distances[*from][*to];
                            self.pheromones[*from][*to] += pheromone_deposit;
                            self.pheromones[*to][*from] += pheromone_deposit;
                        }
                    }
                }
            }
        }
    }
    fn print_results(&self) {
        //println!("Pheromone matrix:");
        //for row in self.pheromones.iter() {
        //    for pheromone in row.iter() {
        //        print!("{:.2} ", pheromone);
        //    }
        //    println!();
        //}

        if !self.city_names.is_empty() {
            let mut best_path_cities = vec![];
            for city in &self.best_path {
                best_path_cities.push(self.city_names[&city].clone());
            }
            println!("Best path: {:?}", self.best_path);
            println!("Best path: {:?}", best_path_cities);
        } else {
            println!("Best path: {:?}", self.best_path);
        }
        println!("Best distance: {:.2}", self.best_distance);
    }
    fn calculate_average_distance(ants: &Vec<Ant>) -> f64 {
        let total_distance: f64 = ants
            .iter()
            .map(|ant| ant.distance_traveled)
            .sum();
        total_distance / (ants.len() as f64)
    }
    fn new(distances: Vec<Vec<f64>>, city_names: Option<HashMap<usize, String>>) -> Self {
        let city_names = city_names.unwrap_or(HashMap::new());
        let cities = (0..distances.len()).collect();
        let best_distance = f64::MAX;
        let best_path = vec![];
        let pheromone_value = 4.0;
        let pheromones = vec![vec![0.5; distances.len()]; distances.len()];
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
            distances,
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

    pub fn new_from_file(file_path: &str) -> Result<AcoModel, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut num_cities = 0;
        let mut distances = vec![];

        for line in reader.lines() {
            let line = line?;
            if line.starts_with("#") || line.trim().is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if num_cities == 0 {
                num_cities = parts.len();
            }
            let row: Vec<f64> = parts
                .iter()
                .map(|&x| x.parse().unwrap())
                .collect();
            distances.push(row);
        }
        if distances.len() != num_cities {
            return Err("The number of rows does not match the number of cities".into());
        }
        Ok(Self::new(distances, None))
    }
    pub fn new_from_excel(
        file_path: &str,
        sheet: Option<&str>
    ) -> Result<AcoModel, Box<dyn Error>> {
        //for this I am strictly assuming a format of |Name of City/Place| Longitude| Latitude| ... where the data I'll be operating on is in the first 3 columns
        // I guess this could be optimised heavily but I am not trying to write pandas for rust
        // probably pola.rs would be the solution here, but I want to limit the library count and if I can get it working without learning a new crate that is preferable.
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
                        DataType::String(val) =>
                            val
                                .parse::<f64>()
                                .unwrap_or_else(|_| {
                                    panic!("Expected a float value for longitude found String({})", val)
                                }),
                        _ => panic!("Expected a float value for longitude found {:?}", &row[1]),
                    };

                    // Extract latitude
                    let latitude = match &row[2] {
                        DataType::Float(val) => *val,
                        DataType::String(val) =>
                            val
                                .parse::<f64>()
                                .unwrap_or_else(|_| {
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
        let mut distances = vec![vec![0.0; num_cities];num_cities];
        for i in 0..num_cities {
            for j in 0..num_cities {
                if i == j {
                    distances[i][j] = 0.0;
                } else {
                    distances[i][j] = calculate_distances(
                        coordinates[i][1],
                        coordinates[j][1],
                        coordinates[i][0],
                        coordinates[j][0]
                    );
                }
            }
        }
        Ok(AcoModel::new(distances, Some(city_indices)))
    }
    pub fn run_model(&mut self) {
        let mut iterations_without_improvement = 0;

        for iteration in 0..self.number_of_iterations {
            // Regenerate ants each cycle
            let mut ants: Vec<Ant> = (0..self.ant_count)
                .map(|_| Ant::new(self.cities.len(), self.final_alpha, self.final_beta))
                .collect();

            for ant in &mut ants {
                ant.generate_path(self);
            }

            let average_distance = AcoModel::calculate_average_distance(&ants);

            // Update pheromones selectively
            self.update_pheromones(&mut ants, average_distance);

            let mut improved = false;
            for ant in &ants {
                if ant.distance_traveled < self.best_distance {
                    println!(
                        "\n 
                    new best at {:?} \n 
                    beating previous best at {:?} \n 
                    on iteration {} \n
                    with alpha of {} \n
                    beta of {} \n
                    ",
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
                if
                    (self.final_alpha <= self.final_alpha / 2.0) &
                    (iterations_without_improvement >= self.number_of_iterations / 10 - 1)
                {
                    self.pheromones = vec![vec![0.5; self.distances.len()]; self.distances.len()]; //added reseting of pheromones, once enough stagnation is reached
                }
            }
        }
        self.print_results()
    }

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
        self.best_distance.clone()
    }
}
