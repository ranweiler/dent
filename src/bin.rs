extern crate clap;
extern crate dent;
extern crate term;
extern crate term_size;

use clap::{App, Arg};
use dent::plot;
use dent::summary::Summary;
use dent::t_test::{SigLevel, TTest, welch_t_test};

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

fn print_summary(s: &Summary) {
    let width = 10;
    let size_width = 6;

    println!(
        "{n:>nw$}  {min:>w$}  {max:>w$}  {med:>w$}  {mean:>w$}  {sd:>w$}  {se:>w$}",
        w = width,
        nw = size_width,
        n = "N",
        min = "Min",
        max = "Max",
        med = "Median",
        mean = "Mean",
        sd= "StdDev",
        se = "StdErr",
    );
    println!(
        "{n:>nw$}  {min:>w$}  {max:>w$}  {med:>w$}  {mean:>w$}  {sd:>w$}  {se:>w$}",
        w = width,
        nw = size_width,
        n = fmt::f(s.size(), width),
        min = fmt::f(s.min(), width),
        max = fmt::f(s.max(), width),
        med = fmt::f(s.median(), width),
        mean = fmt::f(s.mean(), width),
        sd = fmt::f(s.standard_deviation(), width),
        se = fmt::f(s.standard_error(), width),
    );
}

fn print_t_test(t_test: &TTest) {
    let width = 12;
    let reject = if t_test.reject { "yes" } else { "no" };

    println!("{l:>w$} = {v}", w = width, l = "Reject H₀?", v = reject);
    println!("{l:>w$} = {v}", w = width, l = "α", v = t_test.alpha);
    println!("{l:>w$} = {v}", w = width, l = "t", v = t_test.t);
    println!("{l:>w$} = {v}", w = width, l = "Crit. v", v = t_test.crit);
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

fn parse_alpha(arg: &str) -> Result<SigLevel, String> {
    Ok(match arg {
        ".001" => SigLevel::Alpha001,
        ".005" => SigLevel::Alpha005,
        ".01"  => SigLevel::Alpha010,
        ".025" => SigLevel::Alpha025,
        ".05"  => SigLevel::Alpha050,
        ".1"   => SigLevel::Alpha100,
        _ => return Err(format!("Invalid value for α: {:?}", arg))
    })
}

fn summarize_stdin(lax_parsing: bool) -> Result<Summary, Box<error::Error>> {
    let stdin = io::stdin();
    let data = read_data(stdin.lock(), lax_parsing)?;

    Ok(Summary::new(&data)?)
}

fn display_t_test(
    summary1: &Summary,
    summary2: &Summary,
    alpha: SigLevel,
    draw_plot: bool,
    width: usize,
    ascii: bool,
) {
    let t_test = welch_t_test(&summary1, &summary2, alpha);

    if draw_plot {
        let p = ok!(plot::comparison_plot(&[summary1, summary2], width, ascii, true));
        println!("{}\n", p);
    }

    print_summary(&summary1);
    println!();
    print_summary(&summary2);
    println!();
    print_t_test(&t_test);
}

fn display_summaries(
    summaries: &[Summary],
    draw_plot: bool,
    width: usize,
    ascii: bool,
) {
    if draw_plot {
        let summary_refs: Vec<&Summary> = summaries
            .iter()
            .collect();

        let plot = ok!(plot::comparison_plot(&summary_refs, width, ascii, true));
        println!("{}\n", plot);
    }

    for i in 0..summaries.len() {
        if i > 0 {
            println!();
        }
        print_summary(&summaries[i]);
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
        .arg(Arg::with_name("alpha")
             .short("a")
             .long("alpha")
             .value_name("ALPHA")
             .help("Significance level α")
             .takes_value(true)
             .default_value(".05"))
        .arg(Arg::with_name("lax")
             .long("lax")
             .help("Ignore non-numeric input lines"))
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
            // Has a default value, so we can can unwrap.
            let alpha_arg = matches.value_of("alpha").unwrap_or_else(|| unreachable!());
            let alpha = ok!(parse_alpha(alpha_arg));

            display_t_test(
                &summaries[0],
                &summaries[1],
                alpha,
                draw_plot,
                width,
                ascii,
            );
        }
        _ => {
            display_summaries(&summaries, draw_plot, width, ascii);
        },
    };
}
