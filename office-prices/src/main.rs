extern crate rusty_machine as rm;
extern crate csv;

use rm::learning::lin_reg::LinRegressor;
use rm::linalg::Matrix;
use rm::linalg::Vector;
use rm::learning::SupModel;
use std::cmp;


fn main() {
    let M = 10.0; // maximum possible score

    // train
    let (inputs, expected_outputs) = get_data("./data/training-data.txt");

    let mut linear_model = LinRegressor::default();
    linear_model.train_with_optimization(&inputs, &expected_outputs);

    println!("Parameters: {:?}", linear_model.parameters());

    // test
    let (test_inputs, test_outputs) = get_data("./data/test-data.txt");

    let mut sum_of_distances = 0.0;

    for i in 0..test_inputs.rows() {
        let current_test_input = &test_inputs.select_rows(&[i]);
        let output = linear_model.predict(&current_test_input)[0];

        let distance = (output / test_outputs[i]).abs() / test_outputs[i];

        if distance - 0.1 < 0.0 {
            sum_of_distances += 1.0;
        } else {
            sum_of_distances += 1.0 - (distance - 0.1);
        }
    }

    // calculate the total score
    let score = (sum_of_distances / (test_inputs.rows() as f64)) * M;

    println!("Score {:?}", score);
}


/// Reads the training data from the provided file and returns a tuple where the first parameter is
/// a matrix that contains all feature values and the second parameter is a vector with the expected outputs.
fn get_data(input_file: &str) -> (Matrix<f64>, Vector<f64>) {
    let mut file_reader = csv::Reader::from_file(input_file).unwrap();

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    for record in file_reader.decode() {
        let (input1, input2, target): (f64, f64, f64) = record.unwrap();
        inputs.push(input1);
        inputs.push(input2);
        outputs.push(target);
    }

    (Matrix::new(inputs.len() / 2, 2, inputs), Vector::new(outputs))
}
