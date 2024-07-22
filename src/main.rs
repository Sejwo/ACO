//Rn I'm checking how to commit it nicely to github through VSC

/* very early test of whether anything works, pretty much moved into AcoModel struct
fn main() {
    let path = "src\\sgb128_dist.txt";
    let mut model = AcoModel::new_from_file(path).unwrap();
    model.set_number_of_iterations(10);
    model.set_ant_count(2);
    model.set_init_alpha(1.5);
    model.set_init_beta(2.0);
    model.set_decay(0.3);
    model.set_pheromone_value(30.0);
    println!("Starting model");
    model.run_model();
}
  // test matrix
    let distances = vec![
        vec![0.0, 3.0, 4.0, 2.0, 7.0],
        vec![3.0, 0.0, 4.0, 6.0, 3.0],
        vec![4.0, 4.0, 0.0, 5.0, 8.0],
        vec![2.0, 6.0, 5.0, 0.0, 6.0],
        vec![7.0, 3.0, 8.0, 6.0, 0.0]
    ];

    let mut map = MapForAnts::new(distances);

    // Create ants
    let mut stagnation_counter = 0;
    let num_ants = 10000;
    let mut ants: Vec<Ant> = (0..num_ants).map(|_| Ant::new(map.cities.len(), 1.0, 1.0)).collect();
    let mut continue_iterations = true;
    // Simulate ants' movements
    for i in 0..50 {
        // Number of iterations
        if !continue_iterations {
            break;
        }
        for ant in &mut ants {
            //println!("New ant! \n"); when exploring stagnation
            ant.generate_path(&map);
            //println!("\n");
        }
        map.update_pheromones(&ants, 0.2, 1.0);
        for ant in &ants {
            if ant.distance_traveled < map.best_distance {
                stagnation_counter = 0;
                map.best_distance = ant.distance_traveled;
                map.best_path = ant.visited_cities.clone();
            } else {
                println!("So we are not gucci");
                stagnation_counter += 1;
                if stagnation_counter == 30000 {
                    println!("Stagnacja osiągnięta przy {}-tej generacji ", i);
                    continue_iterations = false;
                    break;
                }
            }
        }
    }

    // Visualization (for demonstration, print the pheromone matrix)
    println!("Pheromone matrix:");
    for row in map.pheromones.iter() {
        for pheromone in row.iter() {
            print!("{:.2} ", pheromone);
        }
        println!();
    }
    println!("Best path: {:?}", map.best_path);
    println!("Best distance: {:.2}", map.best_distance);
}
 */

use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::distributions::Distribution;
use std::io::{ BufRead, BufReader };
use std::fs::File;
use std::error::Error;
use Aco::utils::usize_float_multiplication;
//next step im considering is making a stack struct for the ants, 
//since each ant will only use a single array of known size and type by the time of intialisation of the ant
//with that essentially I'll be able to maybe speed up each operation around saving a path for each ant, now i could probably do it by just using arrays since all sizes are known
//but truth be told based off this https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html and this https://doc.rust-lang.org/book/ch03-02-data-types.html but I got kind of intimidated
//what if I use the default stack? I understand stack as essentially an array right? So I push i32 of 1 on the stack, then I push another i32 of 2, if I want to access the 1st value do I have to pop the second?
//22.07.24 ye, so essentially i can't use arrays since their size needs to be known at compile time and since all the vectors are dependant on the distances matrix it's Joever 
//what I may do instead since I'm unsure if the stack can be even optimal is writing another parser for data which takes long/lats and creates a matrix based on them, or even a nice map
pub struct Ant {
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
        let mut path_taken = vec![];
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
        if total_probability == 0.0{
            return self.start_city;
        }
        let dist = WeightedIndex::new(&row_probabilities).expect("problem jakiś no");
        let chosen = model.cities[dist.sample(&mut rng)];
        chosen
    }
}

struct AcoModel {
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
}

impl AcoModel {
    fn new(distances: Vec<Vec<f64>>) -> Self {
        let cities = (0..distances.len()).collect();
        let best_distance = f64::MAX;
        let best_path = vec![];
        let pheromone_value = 1.0;
        let pheromones = vec![vec![0.5; distances.len()]; distances.len()];
        let number_of_iterations = 10;
        let decay = 1.0;
        let ant_count = 100;
        let init_alpha = 1.0;
        let init_beta = 1.0;
        let final_alpha = init_alpha;
        let final_beta = init_beta;
        let alpha_scaling = 0.9;
        let beta_scaling = 1.1;
        Self {
            cities,
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
        }
    }

    fn new_from_file(file_path: &str) -> Result<AcoModel, Box<dyn Error>> {
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

        Ok(Self::new(distances))
    }
    fn update_pheromones(&mut self, ants: &Vec<Ant>, average_distance: f64) {
        // Evaporation step
        for row in self.pheromones.iter_mut() {
            for pheromone in row.iter_mut() {
                *pheromone *= self.decay;
            }
        }

        // Deposit step, only for ants with distance less than the average
        for ant in ants {
            if ant.distance_traveled < average_distance {
                for window in ant.path_taken.windows(2) {
                    if let [from, to] = window {
                        if from != to {
                            let pheromone_deposit = self.pheromone_value / self.distances[*from][*to];
                            self.pheromones[*from][*to] += pheromone_deposit;
                            self.pheromones[*to][*from] += pheromone_deposit;
                        }
                    }
                }
            }
        }
    }
    fn run_model(&mut self) {
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
            self.update_pheromones(&ants, average_distance);
    
            let mut improved = false;
            for ant in &ants {
                if ant.distance_traveled < self.best_distance {
                    println!(
                    "new best at {:?} \n 
                    beating previous best at {:?} \n 
                    on iteration {} \n
                    with alpha of {} \n
                    beta of {} \n
                    and current pheromone matrix {:?}
                    ", ant.distance_traveled, self.best_distance, iteration, self.final_alpha, self.final_beta, self.pheromones);
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
                }
            }
        }
    }
    fn print_results(&self) {
        println!("Pheromone matrix:");
        for row in self.pheromones.iter() {
            for pheromone in row.iter() {
                print!("{:.2} ", pheromone);
            }
            println!();
        }
        println!("Best path: {:?}", self.best_path);
        println!("Best distance: {:.2}", self.best_distance);
    }

    fn set_number_of_iterations(&mut self, number_of_iterations: usize) {
        self.number_of_iterations = number_of_iterations;
    }

    fn set_ant_count(&mut self, ant_count: usize) {
        self.ant_count = ant_count;
    }

    fn set_pheromone_value(&mut self, pheromone_value: f64) {
        self.pheromone_value = pheromone_value;
    }

    fn set_init_alpha(&mut self, init_alpha: f64) {
        self.init_alpha = init_alpha;
        self.final_alpha = init_alpha;
    }

    fn set_init_beta(&mut self, init_beta: f64) {
        self.init_beta = init_beta;
        self.final_beta = init_beta;
    }

    fn set_decay(&mut self, decay: f64) {
        self.decay = decay;
    }
    fn set_alpha_beta_scaling(&mut self, alpha: f64, beta: f64) {
        self.alpha_scaling = alpha;
        self.beta_scaling = beta;
    }
    fn calculate_average_distance(ants: &Vec<Ant>) -> f64 {
        let total_distance: f64 = ants.iter().map(|ant| ant.distance_traveled).sum();
        total_distance / ants.len() as f64
    }
}

fn main() {
    //let path = "src\\fri26_d.txt";
    let path = "src\\sgb128_dist.txt"; //something i realised around this file, pheromone values should either be really high(or the distance values could be normalised to floats <100)
    //let path = "src\\five_d.txt";
    //let path = "src\\gr17_d.txt";
    let mut model = AcoModel::new_from_file(path).unwrap();
    model.set_number_of_iterations(50);
    model.set_ant_count(10000);
    model.set_init_alpha(2.0);
    model.set_init_beta(1.0);
    model.set_decay(0.5);
    model.set_pheromone_value(4.0); //because this is by any means an arbitrary number but with increments of 5-14 this just didn't make sense
    println!("Starting model");
    model.run_model();
    model.print_results();
}
