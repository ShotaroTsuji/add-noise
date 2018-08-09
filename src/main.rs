extern crate clap;
extern crate csv;
extern crate either;

extern crate add_noise;

use clap::{App, Arg};

use either::Either;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::StdinLock;

fn open_file_or_stdin<'a, T: AsRef<std::path::Path>>(
    path: &Option<T>,
    stdin: &'a io::Stdin,
) -> Either<BufReader<File>, StdinLock<'a>> {
    match path {
        Some(path) => {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            Either::Left(reader)
        }
        None => Either::Right(stdin.lock()),
    }
}

fn read_csv_file<R: std::io::BufRead>(stream: R) -> Vec<Vec<f64>> {
    let mut data = Vec::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(stream);
    let mut record = csv::StringRecord::new();

    loop {
        let result = reader.read_record(&mut record).expect("CSV reader error");
        if result {
            let row: Vec<f64> = record
                .iter()
                .map(|s| s.parse::<f64>().expect("Each field must be a float"))
                .collect();
            data.push(row);
        } else {
            break;
        }
    }

    data
}

fn main() {
    let matches = App::new("add-noise")
        .version("0.1.0")
        .author("Shotaro Tsuji <tsuji@sat.t.u-tokyo.ac.jp>")
        .about("add noise to data")
        .arg(
            Arg::with_name("RATIO")
                .short("r")
                .long("ratio")
                .required(true)
                .takes_value(true)
                .help("Noise power ratio to signal power"),
        )
        .arg(Arg::with_name("INPUT").index(1).help("Input file path"))
        .long_about(
            "\
Adds noise to the input data. The noise level is given by the `--ratio` option.
The noise level is the noise power ratio to the signal power. The signal power
is defined as the variance of the input signal. The noise obeys the normal
distribution N(0, ratio*power).
The input file is a text file of rows of float numbers. Each row is a comma-
separated string of float numbers.",
        )
        .get_matches();

    let ratio = matches
        .value_of("RATIO")
        .expect("Ratio must be specified")
        .parse::<f64>()
        .expect("Ratio must be a float.");

    let inputpath = matches.value_of("INPUT");

    eprintln!("ratio: {:?}", ratio);
    eprintln!("input: {:?}", inputpath);

    let stdin = io::stdin();
    let input = open_file_or_stdin(&inputpath, &stdin);

    let mut data = read_csv_file(input);

    add_noise::add_noise(&mut data[..], ratio);

    for row in data.iter() {
        for j in 0..row.len() {
            let delim = if j < row.len() - 1 { "," } else { "\n" };
            print!("{}{}", row[j], delim);
        }
    }
}
