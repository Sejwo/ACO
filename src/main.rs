use aco::structs::{ AcoModel };
fn main() {
    //let path = "fri26_d.txt";
    //let path = "sgb128_dist.txt";
    //let path = "five_d.txt";
    //let path = "gr17_d.txt";
    //let path = "src\\test_excel.xlsx";
    let path = "tests//dantzig42_d.txt";
    //let mut model = AcoModel::new_from_excel(path, Some("Sheet1")).expect(
    //    "Failed to initate model: "
    //);
    let mut model = AcoModel::new_from_file(path).expect("Failed to initiate model: ");
    model.set_number_of_iterations(120);
    model.set_ant_count(20000);
    model.set_init_alpha(5.5);
    model.set_init_beta(1.5);
    model.set_decay(0.85);
    model.set_pheromone_value(4.0); //apparently when i fixed the code the number here doesn't really matter so that's good
    model.set_rank_limit(20000.0*0.1 as u32); // since i added rank limits(how many best ants will get to leave their pheromones i am including this and creating a branch)
    model.run_model();
}
