#[macro_use] pub mod kat;

pub mod fs {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub fn read_data(path: &str) -> Vec<f64> {
        let f = File::open(path).unwrap();
        let r = BufReader::new(f);

        let data: Vec<f64> = r
            .lines()
            .map(|l| l.unwrap().parse().unwrap())
            .collect();

        data

    }
}
