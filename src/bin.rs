extern crate clap;
extern crate dent;

use clap::{App, Arg};
use dent::plot;
use dent::summary::Summary;
use dent::t_test::{SigLevel, TTest, welch_t_test};

use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead, BufReader};


fn print_summary(s: &Summary) {
    println!("N\tMin\tMax\tMedian\tMean\tStdDev\tStdErr");
    println!(
        "{}\t{:0.2}\t{:0.2}\t{:0.2}\t{:0.2}\t{:0.2}\t{:0.2}",
        s.size(),
        s.min(),
        s.max(),
        s.median(),
        s.mean(),
        s.standard_deviation(),
        s.standard_error(),
    );
}

fn print_t_test(t_test: &TTest) {
    println!("T\tDF\tAlpha\tCrit\tRejectNull");
    println!(
        "{:0.3}\t{}\t{:0.3}\t{:0.3}\t{}",
        t_test.t,
        t_test.df,
        t_test.alpha,
        t_test.crit,
        t_test.reject,
    );
}

fn read_file(path: &str) -> Summary {
    let p = Path::new(path);
    let f = File::open(p).ok().unwrap();
    let r = BufReader::new(f);

    let data: Vec<f64> = r
        .lines()
        .map(|l| l.ok().unwrap().parse().unwrap())
        .collect();

    Summary::new(&data).unwrap()
}

fn parse_alpha(arg: &str) -> SigLevel {
    match arg {
        ".001" => SigLevel::Alpha001,
        ".005" => SigLevel::Alpha005,
        ".01"  => SigLevel::Alpha010,
        ".025" => SigLevel::Alpha025,
        ".05"  => SigLevel::Alpha050,
        ".1"   => SigLevel::Alpha100,
        _ => panic!(),
    }
}

fn summarize_stdin() {
    let stdin = io::stdin();

    let data: Vec<f64> = stdin
        .lock()
        .lines()
        .map(|l| l.ok().unwrap().parse().unwrap())
        .collect();

    let s = Summary::new(&data).unwrap();
    println!("{}", plot::summary_plot(&s, 80));
    println!();
    print_summary(&s);
}

fn t_test_files(file1: &str, file2: &str, alpha: SigLevel) {
    let s1 = read_file(file1);
    let s2 = read_file(file2);

    let t_test = welch_t_test(&s1, &s2, alpha);

    println!("{}", plot::comparison_plot(&s1, &s2, 80));
    println!();
    print_summary(&s1);
    println!();
    print_summary(&s2);
    println!();
    print_t_test(&t_test);
}

fn main() {
    let matches = App::new("dent")
        .version("0.1.0")
        .author("Joe Ranweiler <joe@lemma.co>")
        .about("A tiny tool for t-tests &c.")
        .arg(Arg::with_name("stdin")
             .short("s")
             .long("stdin")
             .help("Read and summarize data from stdin."))
        .arg(Arg::with_name("file1")
             .index(1)
             .value_name("FILE1")
             .takes_value(true)
             .required_unless("stdin")
             .help("1st file of sample data."))
        .arg(Arg::with_name("file2")
             .index(2)
             .value_name("FILE2")
             .takes_value(true)
             .required_unless("stdin")
             .help("2nd file of sample data."))
        .arg(Arg::with_name("alpha")
             .short("a")
             .long("alpha")
             .value_name("ALPHA")
             .help("Significance level α")
             .takes_value(true)
             .default_value(".05"))
        .get_matches();

    let use_stdin = matches.is_present("stdin");

    if use_stdin {
        summarize_stdin();
    } else {
        let alpha = parse_alpha(matches.value_of("alpha").unwrap());
        let file1 = matches.value_of("file1").unwrap();
        let file2 = matches.value_of("file2").unwrap();

        t_test_files(file1, file2, alpha);
    }
}
