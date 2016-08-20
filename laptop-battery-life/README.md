# Laptop Battery Life

[Exercise from Hackerrank](https://www.hackerrank.com/challenges/battery) to predict how long the battery of a notebook will last based on the charge time.

## Solution

The battery lifetime _b_ is linearly dependent on the charge time _c_.
⇒ _b = 2c_

For solving the problem linear regression is applied. The final score is _10-X_, where _X_ is the sum of the distances from the expected answer of each test case.

Although the predicted results using linear regression are pretty accurate, using _b = 2c_ for prediction would result in 100% accuracy.
