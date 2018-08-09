add-noise
===

A CLI program that adds noise to the input data.

## Usage 

Adds noise to the input data. The noise level is given by the `--ratio` option.
The noise level is the noise power ratio to the signal power. The signal power
is defined as the variance of the input signal. The noise obeys the normal
distribution N(0, ratio\*power).
The input file is a text file of rows of float numbers. Each row is a comma-
separated string of float numbers.

```
USAGE:
    add-noise --ratio <RATIO> [INPUT]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -r, --ratio <RATIO>    Noise power ratio to signal power

ARGS:
    <INPUT>    Input file path
```

## Author

Shotaro Tsuji <tsuji@sat.t.u-tokyo.ac.jp>
