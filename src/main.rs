#[macro_use]
extern crate clap;
extern crate csv;
extern crate either;

extern crate add_noise;

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
    let matches = clap_app!(myapp =>
        (version: "0.1.0")
        (author: "Shotaro Tsuji <tsuji@sat.t.u-tokyo.ac.jp>")
        (about: "add noise to data")
        (@arg AMPLITUDE: -a --amplitude +required +takes_value "Noise amplitude ratio")
        (@arg INPUT: "Input file path")
    ).get_matches();

    let amplitude = matches
        .value_of("AMPLITUDE")
        .expect("Amplitude must be specified")
        .parse::<f64>()
        .expect("Percentage must be a float.");

    let inputpath = matches.value_of("INPUT");

    eprintln!("amplitude: {:?}", amplitude);
    eprintln!("inputpath: {:?}", inputpath);

    let stdin = io::stdin();
    let input = open_file_or_stdin(&inputpath, &stdin);
    eprintln!("input: {:?}", input);

    let mut data = read_csv_file(input);

    add_noise::add_noise(&mut data[..], amplitude);

    for row in data.iter() {
        for j in 0..row.len() {
            let delim = if j < row.len()-1 { "," } else { "\n" };
            print!("{}{}", row[j], delim);
        }
    }
}
