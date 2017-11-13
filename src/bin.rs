extern crate clap;
extern crate dent;
extern crate term;
extern crate term_size;

use clap::{App, Arg};
use dent::plot;
use dent::summary::Summary;
use dent::t_test::{TTest, welch_t_test};

use std::error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod fmt;
mod log;


macro_rules! ok {
    ($r: expr) => {
        match $r {
            Ok(t) => t,
            Err(e) => {
                log::error(&format!("{}", e));
                std::process::exit(1);
            }
        }
    }
}

fn print_summary(s: &Summary, outliers: bool) {
    let width = 10;
    let size_width = 6;

    if outliers {
        println!(
            "{n:>nw$}  {min:>w$}  {q1:>w$}  {med:>w$}  {q3:>w$}  {max:>w$}  {mean:>w$}  {std:>w$}",
            w = width,
            nw = size_width,
            n = "Size",
            min = "Min",
            q1 = "Q1",
            med = "Median",
            q3 = "Q3",
            max = "Max",
            mean = "Mean",
            std = "Std Dev",
        );
        println!(
            "{n:>nw$}  {min:>w$}  {q1:>w$}  {med:>w$}  {q3:>w$}  {max:>w$}  {mean:>w$}  {std:>w$}",
            w = width,
            nw = size_width,
            n = fmt::f(s.size(), width),
            min = fmt::f(s.min(), width),
            q1 = fmt::f(s.lower_quartile(), width),
            med = fmt::f(s.median(), width),
            q3 = fmt::f(s.upper_quartile(), width),
            max = fmt::f(s.max(), width),
            mean = fmt::f(s.mean(), width),
            std = fmt::f(s.standard_deviation(), width),
        );
    } else {
        println!(
            "{n:>nw$}  {min:>w$}  {q1:>w$}  {med:>w$}  {q3:>w$}  {max:>w$}  {mean:>w$}  {std:>w$}",
            w = width,
            nw = size_width,
            n = "Size",
            min = "Min Adj",
            q1 = "Q1",
            med = "Median",
            q3 = "Q3",
            max = "Max Adj",
            mean = "Mean",
            std = "Std Dev",
        );
        println!(
            "{n:>nw$}  {min:>w$}  {q1:>w$}  {med:>w$}  {q3:>w$}  {max:>w$}  {mean:>w$}  {std:>w$}",
            w = width,
            nw = size_width,
            n = fmt::f(s.size(), width),
            min = fmt::f(s.min_adjacent(), width),
            q1 = fmt::f(s.lower_quartile(), width),
            med = fmt::f(s.median(), width),
            q3 = fmt::f(s.upper_quartile(), width),
            max = fmt::f(s.max_adjacent(), width),
            mean = fmt::f(s.mean(), width),
            std = fmt::f(s.standard_deviation(), width),
        );
    }
}

fn print_t_test(t_test: &TTest, s1: &Summary, s2: &Summary) {
    let width = 12;

    println!("{l:>w$} = {v} ± {se}", w = width, l = "m₁ ± SE", v = s1.mean(), se = s1.standard_error());
    println!("{l:>w$} = {v} ± {se}", w = width, l = "m₂ ± SE", v = s2.mean(), se = s2.standard_error());
    println!("{l:>w$} = {v}", w = width, l = "t", v = t_test.t);
    println!("{l:>w$} = {v}", w = width, l = "p", v = t_test.p);
    println!("{l:>w$} = {v}", w = width, l = "DF", v = t_test.df);
}

fn summarize_file(path: &str, lax_parsing: bool) -> Result<Summary, Box<error::Error>> {
    let f = File::open(path).or_else(|e| {
        log::error(&format!("Could not open file: {:?}", path));
        Err(e)
    })?;
    let reader = BufReader::new(f);

    let data = read_data(reader, lax_parsing)?;

    Ok(Summary::new(&data)?)
}

fn read_data<R>(reader: R, lax_parsing: bool) -> Result<Vec<f64>, Box<error::Error>>
    where R: BufRead {
    let mut data: Vec<f64> = vec![];

    for l in reader.lines() {
        let s = l?.trim().to_string();

        if s.is_empty() {
            continue;
        }

        match s.parse() {
            Ok(d) => data.push(d),
            err => if !lax_parsing { err?; }
        }
    }

    Ok(data)
}

fn summarize_stdin(lax_parsing: bool) -> Result<Summary, Box<error::Error>> {
    let stdin = io::stdin();
    let data = read_data(stdin.lock(), lax_parsing)?;

    Ok(Summary::new(&data)?)
}

fn display_t_test(
    summary1: &Summary,
    summary2: &Summary,
    draw_plot: bool,
    width: usize,
    ascii: bool,
    outliers: bool,
) {
    let t_test = ok!(welch_t_test(&summary1, &summary2));

    if draw_plot {
        let p = ok!(plot::comparison_plot(&[summary1, summary2], width, ascii, true, outliers));
        println!("{}\n", p);
    }

    print_summary(&summary1, outliers);
    println!();
    print_summary(&summary2, outliers);
    println!();
    print_t_test(&t_test, &summary1, &summary2);
}

fn display_summaries(
    summaries: &[Summary],
    draw_plot: bool,
    width: usize,
    ascii: bool,
    outliers: bool,
) {
    if draw_plot {
        let summary_refs: Vec<&Summary> = summaries
            .iter()
            .collect();

        let plot = ok!(plot::comparison_plot(&summary_refs, width, ascii, true, true));
        println!("{}\n", plot);
    }

    for i in 0..summaries.len() {
        if i > 0 {
            println!();
        }
        print_summary(&summaries[i], outliers);
    }
}

fn main() {
    let matches = App::new("dent")
        .version("0.3.0")
        .author("Joe Ranweiler <joe@lemma.co>")
        .about("A tiny tool for t-tests &c.")
        .arg(Arg::with_name("stdin")
             .short("s")
             .long("stdin")
             .help("Read and summarize data from stdin"))
        .arg(Arg::with_name("files")
             .multiple(true)
             .value_name("FILES")
             .takes_value(true)
             .required_unless("stdin")
             .help("Path to one or more files of sample data"))
        .arg(Arg::with_name("lax")
             .long("lax")
             .help("Ignore non-numeric input lines"))
        .arg(Arg::with_name("plot_outliers")
             .long("outliers")
             .help("Include outliers and use min/max for outer fences of boxplot"))
        .arg(Arg::with_name("plot")
             .short("p")
             .long("plot")
             .help("Print standard boxplots"))
        .arg(Arg::with_name("ascii")
             .long("ascii")
             .help("Use only ASCII characters in boxplots"))
        .arg(Arg::with_name("width")
             .short("w")
             .long("width")
             .value_name("WIDTH")
             .takes_value(true)
             .help("Width of boxplot"))
        .get_matches();

    let ascii = matches.is_present("ascii");
    let lax_parsing = matches.is_present("lax");
    let draw_plot = matches.is_present("plot");
    let use_stdin = matches.is_present("stdin");
    let outliers = matches.is_present("plot_outliers");

    let width = matches
        .value_of("width")
        .and_then(|w| w.parse::<usize>().ok())
        .or(term_size::dimensions().map(|(w, _)| w))
        .unwrap_or(80);

    let summaries = if use_stdin {
        vec![ok!(summarize_stdin(lax_parsing))]
    } else {
        // Required if `stdin` is not present, so we can unwrap.
        matches.values_of("files")
            .unwrap_or_else(|| unreachable!())
            .map(|f| ok!(summarize_file(f, lax_parsing)))
            .collect()
    };

    match summaries.len() {
        0 => unreachable!(),
        // We want match 1 with the case `len()` > 2.
        2 => {
            display_t_test(
                &summaries[0],
                &summaries[1],
                draw_plot,
                width,
                ascii,
                outliers,
            );
        }
        _ => {
            display_summaries(
                &summaries,
                draw_plot,
                width,
                ascii,
                outliers,
            );
        },
    };
}
