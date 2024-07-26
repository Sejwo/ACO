#[cfg(test)]
mod tests {
    use aco::structs::AcoModel;
    #[test]
    fn five_d_test() {
        let path = "tests\\five_d.txt";
        let mut model_five_d = AcoModel::new_from_file(path).expect("Failed to read file: ");
        model_five_d.set_number_of_iterations(30);
        model_five_d.set_ant_count(1000);
        model_five_d.set_init_alpha(2.0);
        model_five_d.set_init_beta(1.0);
        model_five_d.set_decay(0.5);
        model_five_d.set_pheromone_value(4.0);
        model_five_d.run_model();
        assert_eq!(model_five_d.return_best_result(), 19.0);
    }
    #[test]
    fn gr17_d_test() {
        let path = "tests\\gr17_d.txt";
        let mut model_five_d = AcoModel::new_from_file(path).expect("Failed to read file: ");
        model_five_d.set_number_of_iterations(40);
        model_five_d.set_ant_count(1500);
        model_five_d.set_init_alpha(2.0);
        model_five_d.set_init_beta(1.0);
        model_five_d.set_decay(0.5);
        model_five_d.set_pheromone_value(4.0);
        model_five_d.run_model();
        assert_eq!(model_five_d.return_best_result(), 2085.0);
    }
    #[test]
    fn fri26_d_test() {
        let path = "tests\\fri26_d.txt";
        let mut model_five_d = AcoModel::new_from_file(path).expect("Failed to read file: ");
        model_five_d.set_number_of_iterations(50);
        model_five_d.set_ant_count(2000);
        model_five_d.set_init_alpha(2.0);
        model_five_d.set_init_beta(1.0);
        model_five_d.set_decay(0.5);
        model_five_d.set_pheromone_value(4.0);
        model_five_d.run_model();
        assert_eq!(model_five_d.return_best_result(), 937.0);
    }
    #[test]
    fn dantzig42_d_test() {
        let path = "tests\\dantzig42_d.txt";
        let mut model_five_d = AcoModel::new_from_file(path).expect("Failed to read file: ");
        model_five_d.set_number_of_iterations(75);
        model_five_d.set_ant_count(7500);
        model_five_d.set_init_alpha(2.0);
        model_five_d.set_init_beta(1.0);
        model_five_d.set_decay(0.5);
        model_five_d.set_pheromone_value(4.0);
        model_five_d.run_model();
        assert_eq!(model_five_d.return_best_result(), 699.0);
    }
    #[test]
    fn att48_d_test() {
        let path = "tests\\att48_d.txt";
        let mut model_five_d = AcoModel::new_from_file(path).expect("Failed to read file: ");
        model_five_d.set_number_of_iterations(100);
        model_five_d.set_ant_count(9999);
        model_five_d.set_init_alpha(2.0);
        model_five_d.set_init_beta(1.0);
        model_five_d.set_decay(0.5);
        model_five_d.set_pheromone_value(4.0);
        model_five_d.run_model();
        assert_eq!(model_five_d.return_best_result(), 33523.0);
    }
    #[test]
    fn excel_plain_test() {
        let path = "tests\\regular_excel_test.xlsx";
        let mut model_five_d = AcoModel::new_from_excel(path, Some("Sheet1")).expect("Failed to read file: ");
        model_five_d.set_number_of_iterations(40);
        model_five_d.set_ant_count(4000);
        model_five_d.set_init_alpha(2.0);
        model_five_d.set_init_beta(1.0);
        model_five_d.set_decay(0.5);
        model_five_d.set_pheromone_value(4.0);
        model_five_d.run_model();
        assert!(model_five_d.return_best_result() <= 4308.7828360182575); //since i don't know the actual optimum i will be a little bit leanient here
    }
}