use aco::structs::{ AcoModel };
fn main() {
    //let path = "fri26_d.txt";
    //let path = "sgb128_dist.txt";
    //let path = "five_d.txt";
    //let path = "gr17_d.txt";
    //let mut model = AcoModel::new_from_file(path).expect("Failed to initiate model: ");
    let path = "src\\test_excel.xlsx";
    let mut model = AcoModel::new_from_excel(path, Some("Sheet1")).expect(
        "Failed to initate model: "
    );
    model.set_number_of_iterations(30);
    model.set_ant_count(10000);
    model.set_init_alpha(2.0);
    model.set_init_beta(1.0);
    model.set_decay(0.5);
    model.set_pheromone_value(4.0); //apparently when i fixed the code the number here doesn't really matter so that's good
    model.run_model();
}
