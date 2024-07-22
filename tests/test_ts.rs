//forgot to comment this, absolutely outdated and integrated in the src file, i will be shuffling them around as I go


use std::fs::File;
use std::io::{ BufRead, BufReader };

struct TSPInstance {
    cities: Vec<usize>,
    distances: Vec<Vec<f64>>,
    optimal_solution: Vec<usize>,
}

impl TSPInstance {
    fn new(num_cities: usize, distances: Vec<Vec<f64>>, optimal_solution: Vec<usize>) -> Self {
        let cities = (0..num_cities).collect();
        Self {
            cities,
            distances,
            optimal_solution,
        }
    }

    fn from_file(file_path: &str) -> Option<Self> {
        let file = File::open(file_path).ok()?;
        let reader = BufReader::new(file);
        let mut num_cities = 0;
        let mut distances = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            if line.starts_with("#") {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            if num_cities == 0 {
                num_cities = parts.len();
            }
            let row: Vec<f64> = parts
                .iter()
                .map(|&x| x.parse().unwrap())
                .collect();
            distances.push(row);
        }

        let optimal_solution = vec![]; // Placeholder for optimal solution (if available)

        Some(Self::new(num_cities, distances, optimal_solution))
    }

    fn get_cities(&self) -> &Vec<usize> {
        &self.cities
    }

    fn get_distances(&self) -> &[Vec<f64>] {
        &self.distances
    }

    fn get_optimal_solution(&self) -> &[usize] {
        &self.optimal_solution
    }
    fn optimal_solution_mut(&mut self) -> &mut Vec<usize> {
        &mut self.optimal_solution
    }
}
fn main() {
    let file_path = "five_d.txt";
    if let Some(mut tsp_instance) = TSPInstance::from_file(file_path) {
        *tsp_instance.optimal_solution_mut() = vec![1, 3, 2, 5, 4];
        println!("Parsed TSPInstance:");
        println!("Cities: {:?}", tsp_instance.get_cities());
        println!("\n");
        println!("Distances: {:?}", tsp_instance.get_distances());
        println!("Optimal Solution: {:?}", tsp_instance.get_optimal_solution());
    }
    //let file_path = "tests\\sgb128_dist.txt";
    //if let Some(tsp_instance) = TSPInstance::from_file(file_path) {
    // If `from_file` returns `Some(TSPInstance)`, unwrap the TSPInstance
    //    println!("Parsed TSPInstance:");
    //    println!("Cities: {:?}", tsp_instance.cities());
    //    println!("\n");
    //    println!("Distances: {:?}", tsp_instance.distances());
    //    println!("Optimal Solution: {:?}", tsp_instance.optimal_solution());
    //} else {
    //    // If `from_file` returns `None`, handle the error
    //    eprintln!("Failed to parse the input file.");
    //}
}
