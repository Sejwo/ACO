use aco::structs::{ AcoModel };
use aco::utils::{read_from_tsp_file, read_from_txt_file};
fn main() {
    let path = "tests\\bays29.tsp";
    //let path = "fri26_d.txt";
    //let path = "sgb128_dist.txt";
    //let path = "tests\\five_d.txt";
    //let path = "gr17_d.txt";
    //let path = "src\\test_excel.xlsx";
    //let path = "tests//dantzig42_d.txt";
    //let mut model: AcoModel = read_from_txt_file(path);
    let mut model = read_from_tsp_file(path);
    model.set_number_of_iterations(1000);
    model.set_ant_count(2500);
    model.set_init_alpha(4.5);
    model.set_init_beta(1.5);
    model.set_decay(0.65);
    model.set_pheromone_value(4.0); //apparently when i fixed the code the number here doesn't really matter so that's good
    model.set_rank_limit((2500.0*0.1) as u32); // since i added rank limits(how many best ants will get to leave their pheromones i am including this and creating a branch)
    model.run_model();
}
