use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


pub fn read_data(path: &str) -> Vec<f64> {
    let p = Path::new(path);
    let f = File::open(p).ok().unwrap();
    let r = BufReader::new(f);

    let data: Vec<f64> = r
        .lines()
        .map(|l| l.ok().unwrap().parse().unwrap())
        .collect();

    data
}

#[derive(Debug, Default)]
pub struct KnownSummary {
    pub src: String,
    pub size: f64,
    pub min: f64,
    pub max: f64,
    pub median: f64,
    pub mean: f64,
    pub lower_quartile: f64,
    pub upper_quartile: f64,
    pub standard_deviation: f64,
    pub standard_error: f64,
    pub variance: f64,
}

impl KnownSummary {
    pub fn new(path: &str) -> Self {
        let p = Path::new(path);
        let f = File::open(p).ok().unwrap();
        let r = BufReader::new(f);

        let mut known = KnownSummary::default();
        let mut keys_read: HashSet<String> = HashSet::new();

        for l in r.lines() {
            let pieces: Vec<String> = l.ok().unwrap().split('\t').map(|s| s.to_string()).collect();
            assert_eq!(pieces.len(), 2, "Invalid line in known answer file");

            let key = pieces[0].to_string();
            let val = pieces[1].to_string();

            match key.as_ref() {
                "src" =>
                    known.src = val,
                "lower_quartile" =>
                    known.lower_quartile = val.parse::<f64>().unwrap(),
                "max" =>
                    known.max = val.parse::<f64>().unwrap(),
                "mean" =>
                    known.mean = val.parse::<f64>().unwrap(),
                "median" =>
                    known.median = val.parse::<f64>().unwrap(),
                "min" =>
                    known.min = val.parse::<f64>().unwrap(),
                "n" =>
                    known.size = val.parse::<f64>().unwrap(),
                "sem" =>
                    known.standard_error = val.parse::<f64>().unwrap(),
                "std" =>
                    known.standard_deviation = val.parse::<f64>().unwrap(),
                "upper_quartile" =>
                    known.upper_quartile = val.parse::<f64>().unwrap(),
                "var" =>
                    known.variance = val.parse::<f64>().unwrap(),
                _ => panic!(),
            }

            keys_read.insert(key.to_string());
        }

        assert_eq!(keys_read.len(), 11, "Missing lines in known answer file");

        known
    }
}

#[derive(Debug, Default)]
pub struct KnownTTest {
    pub src1: String,
    pub src2: String,
    pub t: f64,
}

impl KnownTTest {
    pub fn new(path: &str) -> Self {
        let p = Path::new(path);
        let f = File::open(p).ok().unwrap();
        let r = BufReader::new(f);

        let mut known = KnownTTest::default();
        let mut keys_read: HashSet<String> = HashSet::new();

        for l in r.lines() {
            let pieces: Vec<String> = l.ok().unwrap().split('\t').map(|s| s.to_string()).collect();
            assert_eq!(pieces.len(), 2, "Invalid line in known answer file");

            let key = pieces[0].to_string();
            let val = pieces[1].to_string();

            match key.as_ref() {
                "src1" => known.src1 = val,
                "src2" => known.src2 = val,
                "t" => known.t = val.parse::<f64>().unwrap(),
                _ => panic!(),
            }

            keys_read.insert(key.to_string());
        }

        assert_eq!(keys_read.len(), 3, "Missing lines in known answer file");

        known
    }
}

#[derive(Debug, Default)]
pub struct KnownLR {
    pub src: String,
    pub slope: f64,
    pub intercept: f64,
    pub r: f64,
    pub p: f64,
    pub se: f64,
}

impl KnownLR {
    pub fn new(path: &str) -> Self {
        let p = Path::new(path);
        let f = File::open(p).ok().unwrap();
        let r = BufReader::new(f);

        let mut known = KnownLR::default();
        let mut keys_read: HashSet<String> = HashSet::new();

        for l in r.lines() {
            let pieces: Vec<String> = l.unwrap().split('\t').map(|s| s.to_string()).collect();
            assert_eq!(pieces.len(), 2, "Invalid line in known answer file");

            let key = pieces[0].to_string();
            let val = pieces[1].to_string();

            match key.as_ref() {
                "src" => known.src = val,
                "slope" => known.slope = val.parse::<f64>().unwrap(),
                "intercept" => known.intercept = val.parse::<f64>().unwrap(),
                "r" => known.r = val.parse::<f64>().unwrap(),
                "p" => known.p = val.parse::<f64>().unwrap(),
                "se" => known.se = val.parse::<f64>().unwrap(),
                _ => panic!("Unknown key in known answer file"),
            }

            keys_read.insert(key.to_string());
        }

        assert_eq!(keys_read.len(), 6, "Missing lines in known answer file");

        known
    }
}

macro_rules! assert_appx_eq {
    ($name:expr, $tolerance:expr, $known:expr, $actual:expr) => {
        let d = ($known - $actual).abs();

        let err = format!("{}: {} and {} differ by {} > {}",
                          $name, $known, $actual, d, $tolerance);

        assert!(d < $tolerance, err);
    };
}

#[macro_export]
macro_rules! summary_kat {
    ($test_name:tt, $name:expr) => {
        #[test]
        fn $test_name() {
            use dent::summary::Summarizer;
            use $crate::common::{KnownSummary, read_data};

            let data_path = format!("{}/{}", "support/data", $name);
            let data = read_data(&data_path);
            let summary = Summarizer::new(&data).unwrap();

            let known_path = format!("{}{}", "support/kat/summary_", $name);
            let known = KnownSummary::new(&known_path);

            let precision = 1e-14;

            assert_appx_eq!("Size", precision,
                            known.size, summary.size());
            assert_appx_eq!("Min", precision,
                            known.min, summary.min());
            assert_appx_eq!("Max", precision,
                            known.max, summary.max());
            assert_appx_eq!("Median", precision,
                            known.median, summary.median());
            assert_appx_eq!("Mean", precision,
                            known.mean, summary.mean());
            assert_appx_eq!("0th percentile", precision,
                            known.min, summary.percentile(0.0).unwrap());
            assert_appx_eq!("Lower quartile", precision,
                            known.lower_quartile, summary.percentile(0.25).unwrap());
            assert_appx_eq!("Upper quartile", precision,
                            known.upper_quartile, summary.percentile(0.75).unwrap());
            assert_appx_eq!("100th percentile", precision,
                            known.max, summary.percentile(1.0).unwrap());
            assert_appx_eq!("Variance", precision,
                            known.variance, summary.unbiased_variance());
            assert_appx_eq!("Standard deviation", precision,
                            known.standard_deviation, summary.standard_deviation());
            assert_appx_eq!("Standard error", precision,
                            known.standard_error, summary.standard_error());
        }
    }
}

#[macro_export]
macro_rules! t_test_kat {
    ($test_name:tt, $name:expr) => {
        #[test]
        fn $test_name() {
            use dent::summary::Summary;
            use dent::t_test::{SigLevel, welch_t_test};
            use $crate::common::{KnownTTest, read_data};

            let known_path = format!("{}/{}", "support/kat", $name);
            let known = KnownTTest::new(&known_path);

            let data_path1 = format!("{}/{}", "support/data", known.src1);
            let data1 = read_data(&data_path1);
            let summary1 = Summary::new(&data1).unwrap();

            let data_path2 = format!("{}/{}", "support/data", known.src2);
            let data2 = read_data(&data_path2);
            let summary2 = Summary::new(&data2).unwrap();

            let t_test = welch_t_test(&summary1, &summary2, SigLevel::Alpha005);

            let precision = 1e-11 ;

            assert_appx_eq!("T statistic", precision,
                            known.t, t_test.t);
        }
    }
}

#[macro_export]
macro_rules! lr_kat {
    ($test_name:tt, $name:expr) => {
        #[test]
        fn $test_name() {
            use dent::lr::LinearRegression;
            use $crate::common::{KnownLR, read_data};

            let x_path = format!("{}/{}-x", "support/data", $name);
            let x = read_data(&x_path);

            let y_path = format!("{}/{}-y", "support/data", $name);
            let y = read_data(&y_path);

            let data: Vec<_> = x.iter().cloned().zip(y).collect();

            let lr = LinearRegression::new(&data).unwrap();

            let known_path = format!("{}/{}", "support/kat", $name);
            let known = KnownLR::new(&known_path);

            let precision = 1e-9;

            assert_appx_eq!("Slope", precision, known.slope, lr.slope());
            assert_appx_eq!("Intercept", precision, known.intercept, lr.intercept());
            assert_appx_eq!("R", precision, known.r, lr.r());
            assert_appx_eq!("Standard Error", 1e-10, known.se, lr.standard_error());

            // We dont compute this right now.
            // assert_appx_eq!("P", precision, known.p, lr.p);
        }
    }
}
