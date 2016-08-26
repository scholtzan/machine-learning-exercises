extern crate rusty_machine;
extern crate csv;


use rusty_machine::linalg::Vector;
use rusty_machine::linalg::Matrix;


fn main() {
    let (gpas, test_results) = get_data("./data/dataset.txt");

    let mut best_score = -1.0;
    let mut best_test = 0;

    for test in 0..test_results.rows() {
        let test_results_row = Vector::new(test_results.select_rows(&[test]).into_vec());
        let score = pearson_correlation(&gpas, &test_results_row);

        if score > best_score {
            best_score = score;
            best_test = test;
        }
    }

    println!("Best Score {:?}, Best Test {:?}", best_score, best_test);
}


/// Calculates the pearson correlation score between two vectors.
///
/// Returns a score between 1 and -1.
/// The higher the score the more similar the two vectors are.
fn pearson_correlation(v1: &Vector<f64>, v2: &Vector<f64>) -> f64 {
    println!("{:?}, {:?}", v1, v2);

    assert!(v1.size() == v2.size());

    fn square(x: f64) -> f64 {
        x * x
    }

    // clone, otherwise when using `apply` -> cannot move out of borrowed content
    let squared_v1 = v1.clone().apply(&square);
    let squared_v2 = v2.clone().apply(&square);
    let dot_product = v1.dot(&v2);
    let n = v1.size() as f64;

    let d = ((squared_v1.sum() - v1.sum().powi(2) / n) * (squared_v2.sum() - v2.sum().powi(2) / n)).sqrt();

    if d == 0.0 {
        0.0
    } else {
        (dot_product - (v1.sum() * v2.sum() / n)) / d
    }
}


/// Reads the training data from the provided file and returns a tuple where the first parameter is
/// a vector that contains all GPAs values and the second parameter is a matrix with the test scores.
fn get_data(input_file: &str) -> (Vector<f64>, Matrix<f64>) {
    let mut file_reader = csv::Reader::from_file(input_file).unwrap();

    let mut gpas = Vec::new();
    let mut test_results = Vec::new();

    for record in file_reader.decode::<Vec<f64>>() {
        if gpas.len() == 0 {
            gpas.append(&mut record.unwrap());
        } else {
            test_results.append(&mut record.unwrap());
        }
    }

    let n = gpas.len();
    let m = test_results.len() / gpas.len();

    (Vector::new(gpas), Matrix::new(n, m, test_results))
}
