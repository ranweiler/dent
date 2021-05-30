#[macro_use] extern crate clap;
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

    let m1 = s1.mean();
    let m2 = s2.mean();
    let se1 = s1.standard_error();
    let se2 = s1.standard_error();

    let del = m2 - m1;
    let se_del = (se1.powi(2) + se1.powi(2)).sqrt();

    println!("{l:>w$} = {v} ± {se}", w = width, l = "m₁ ± SE", v = m1, se = se1);
    println!("{l:>w$} = {v} ± {se}", w = width, l = "m₂ ± SE", v = m2, se = se2);
    println!("{l:>w$} = {v} ± {se}", w = width, l = "m₂ - m₁ ± SE", v = del, se = se_del);
    println!("{l:>w$} = {v}", w = width, l = "p", v = t_test.p);
    println!("{l:>w$} = {v}", w = width, l = "t", v = t_test.t);
    println!("{l:>w$} = {v}", w = width, l = "DF", v = t_test.df);
}

fn summarize_file(path: &str, lax_parsing: bool) -> Result<Summary, Box<dyn error::Error>> {
    let f = File::open(path).or_else(|e| {
        log::error(&format!("Could not open file: {:?}", path));
        Err(e)
    })?;
    let reader = BufReader::new(f);

    let data = read_data(reader, lax_parsing)?;

    Ok(Summary::new(&data)?)
}

fn read_data<R>(reader: R, lax_parsing: bool) -> Result<Vec<f64>, Box<dyn error::Error>>
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

fn summarize_stdin(lax_parsing: bool) -> Result<Summary, Box<dyn error::Error>> {
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

        let plot = ok!(plot::comparison_plot(&summary_refs, width, ascii, true, outliers));
        println!("{}\n", plot);
    }

    for i in 0..summaries.len() {
        if i > 0 {
            println!();
        }
        print_summary(&summaries[i], outliers);
    }
}

fn display_summaries_tsv(summaries: &[Summary], sources: &[&str]) {
    let parts = vec![
        "Source",
        "Size",
        "Mean",
        "Median",
        "StandardDeviation",
        "Variance",
        "StandardError",
        "Min",
        "Max",
        "Range",
        "LowerQuartile",
        "UpperQuartile",
        "IQR",
        "MinAdjacent",
        "MaxAdjacent",
    ];
    let header = parts.join("\t");
    println!("{}", header);

    for (summ, src) in summaries.iter().zip(sources) {
        print_summary_tsv(summ, src);
    }
}

fn print_summary_tsv(summary: &Summary, source: &str) {
    let values = vec![
        summary.size(),
        summary.mean(),
        summary.median(),
        summary.standard_deviation(),
        summary.unbiased_variance(),
        summary.standard_error(),
        summary.min(),
        summary.max(),
        summary.range(),
        summary.lower_quartile(),
        summary.upper_quartile(),
        summary.iqr(),
        summary.min_adjacent(),
        summary.max_adjacent(),
    ];
    let fields: Vec<String> = values.iter().map(|x| format!("{}", x)).collect();
    println!("{}\t{}", source, fields.join("\t"));
}

fn main() {
    let matches = App::new("dent")
        .version(crate_version!())
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
        .arg(Arg::with_name("tsv")
             .long("tsv")
             .help("Print summary data to stdout in TSV format"))
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
    let tsv = matches.is_present("tsv");

    let width = matches
        .value_of("width")
        .and_then(|w| w.parse::<usize>().ok())
        .or(term_size::dimensions().map(|(w, _)| w))
        .unwrap_or(80);

    let (sources, summaries) = if use_stdin {
        (vec!["stdin"], vec![ok!(summarize_stdin(lax_parsing))])
    } else {
        // Required if `stdin` is not present, so we can unwrap.
        let files = matches
            .values_of("files")
            .unwrap_or_else(|| unreachable!());

        let summaries = files.clone().map(|f| ok!(summarize_file(f, lax_parsing))).collect();
        (files.collect(), summaries)
    };

    if tsv {
        return display_summaries_tsv(&summaries, &sources);
    }

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
