extern crate rusty_machine as rm;
extern crate csv;

use rm::learning::lin_reg::LinRegressor;
use rm::linalg::Matrix;
use rm::linalg::Vector;
use rm::learning::SupModel;


fn main() {
    // train
    let (training_inputs, training_targets, maximum_target) = get_training_data("./data/trainingdata.txt");

    let mut linear_model = LinRegressor::default();
    linear_model.train_with_optimization(&training_inputs, &training_targets);

    println!("{:?}", linear_model.parameters());

    // test
    let (test_data, _) = get_data("./data/testdata.txt");
    let mut sum_of_distances = 0.0;

    // calculate distances between determined and expected output
    for &(test_input, test_target) in &test_data {
        let output = linear_model.predict(&Matrix::new(1,1,vec![test_input]));

        if output[0] > maximum_target {
            sum_of_distances += (maximum_target - test_target).abs();
        } else {
            sum_of_distances += (output[0] - test_target).abs();
        }
    }

    // score is 10 - the sum of the distances from the expected answer of each test case
    println!("Score: {:?}", 10.0 - sum_of_distances);
}


/// Transforms the trainging data to a tuple where the first value is a matrix with input values and
/// the second value contains the corresponding target values.
fn get_training_data(filename: &str) -> (Matrix<f64>, Vector<f64>, f64) {
    let (training_data, maximum_target) = get_data(filename);

    // remove all data points where the charge time is longer than required
    let normalized_training_data = training_data.into_iter().filter(|&(_, t)|
        t < maximum_target
    ).collect::<Vec<_>>();

    let (inputs, targets): (Vec<_>, Vec<_>) = normalized_training_data.into_iter().unzip();

    (Matrix::new(inputs.len(), 1, inputs), Vector::new(targets), maximum_target)
}


/// Reads the training data from the input file and stores it in a tuple.
/// The first tuple value contains the charge times as matrix, the second vector represents how long the battery lasted.
fn get_data(filename: &str) -> (Vec<(f64, f64)>, f64) {
    let mut file_reader = csv::Reader::from_file(filename).unwrap();

    let mut training_data = Vec::new();

    let mut maximum_target = 0.0;

    for record in file_reader.decode() {
        let (input, target): (f64, f64) = record.unwrap();
        training_data.push((input, target));

        // determine the maximum time the battery lasted
        if target > maximum_target {
            maximum_target = target;
        }
    }

    (training_data, maximum_target)
}
