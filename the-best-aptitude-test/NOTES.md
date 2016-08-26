# The Best Aptitude Test

The original exercise from Hackerrank can be found [here](https://www.hackerrank.com/challenges/the-best-aptitude-test).
The objective is to determine the test based on its results that is best suited for indicating the final GPA.

## Solution

The Pearson Correlation Coefficient is used to calculate the similarity between the GPA and each test. The test with the highest coefficient is the best indicator for the final GPA.


## Dependencies

* [rusty-machine](https://github.com/AtheMathmo/rusty-machine)
* [rust-csv](https://github.com/BurntSushi/rust-csv)
