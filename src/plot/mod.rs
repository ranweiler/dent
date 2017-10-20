mod figure;

use stamp;
use summary::Summary;


struct Boxplot {
    box_lo: f64,
    box_mid: f64,
    box_hi: f64,
    marker: f64,
    wh_lo: f64,
    wh_hi: f64,
}

impl Boxplot {
    fn from_summary(summary: &Summary) -> Self {
        let range = summary.max() - summary.min();
        let n = |x| (x - summary.min()) / range;

        Boxplot {
            box_lo: n(summary.lower_quartile()),
            box_mid: n(summary.median()),
            box_hi: n(summary.upper_quartile()),
            marker: n(summary.mean()),
            wh_lo: n(summary.min()),
            wh_hi: n(summary.max()),
        }
    }
}

struct BoxplotCols {
    box_lo: usize,
    box_mid: usize,
    box_hi: usize,
    marker: usize,
    wh_lo: usize,
    wh_hi: usize,
}

impl BoxplotCols {
    fn new(data: &Boxplot, width: usize) -> Self {
        let max_col  = (width - 1) as f64;
        let to_col = |x: f64| (x * max_col).floor() as usize;

        BoxplotCols {
            box_lo: to_col(data.box_lo),
            box_mid: to_col(data.box_mid),
            box_hi: to_col(data.box_hi),
            marker: to_col(data.marker),
            wh_lo: to_col(data.wh_lo),
            wh_hi: to_col(data.wh_hi),
        }
    }
}

struct RowChars {
    wh_lo: &'static str,
    wh_lo_box_lo_fill: &'static str,
    box_lo: &'static str,
    box_lo_box_mid_fill: &'static str,
    box_mid: &'static str,
    box_mid_box_hi_fill: &'static str,
    box_hi: &'static str,
    box_hi_wh_hi_fill: &'static str,
    wh_hi: &'static str,
}

impl RowChars {
    pub fn render(&self, row: &mut Vec<String>, cols: &BoxplotCols) {
        row[cols.wh_lo] = self.wh_lo.to_string();
        for i in (cols.wh_lo + 1)..cols.box_lo {
            row[i] = self.wh_lo_box_lo_fill.to_string();
        }
        row[cols.box_lo] = self.box_lo.to_string();
        for i in (cols.box_lo + 1)..cols.box_mid {
            row[i] = self.box_lo_box_mid_fill.to_string();
        }
        row[cols.box_mid] = self.box_mid.to_string();
        for i in (cols.box_mid + 1)..cols.box_hi {
            row[i] = self.box_mid_box_hi_fill.to_string();
        }
        row[cols.box_hi] = self.box_hi.to_string();
        for i in (cols.box_hi + 1)..cols.wh_hi {
            row[i] = self.box_hi_wh_hi_fill.to_string();
        }
        row[cols.wh_hi] = self.wh_hi.to_string();
    }
}

struct BoxplotChars {
    marker: &'static str,
    rows: [RowChars; 3],
}

impl BoxplotChars {
    pub fn render(&self, summary: &Summary, width: usize) -> String {
        let data = Boxplot::from_summary(summary);
        let cols = BoxplotCols::new(&data, width);

        let mut plot = Plot::new(width);

        self.rows[0].render(&mut plot.0, &cols);
        self.rows[1].render(&mut plot.1, &cols);
        self.rows[2].render(&mut plot.2, &cols);

        let no_marker = plot.render();

        let base = stamp::Stamp::new(&no_marker).unwrap();
        let marker = stamp::Stamp::new(self.marker).unwrap();
        let layered = base.layer(&marker, cols.marker, 1).unwrap();

        layered.render()
    }
}

static ASCII_CHARS: BoxplotChars = BoxplotChars {
    marker: "x",
    rows: [
        RowChars {
            wh_lo: " ",
            wh_lo_box_lo_fill: " ",
            box_lo: "+",
            box_lo_box_mid_fill: "-",
            box_mid: "+",
            box_mid_box_hi_fill: "-",
            box_hi: "+",
            box_hi_wh_hi_fill: " ",
            wh_hi: " ",
        },
        RowChars {
            wh_lo: "|",
            wh_lo_box_lo_fill: "-",
            box_lo: "|",
            box_lo_box_mid_fill: " ",
            box_mid: "|",
            box_mid_box_hi_fill: " ",
            box_hi: "|",
            box_hi_wh_hi_fill: "-",
            wh_hi: "|",
        },
        RowChars {
            wh_lo: " ",
            wh_lo_box_lo_fill: " ",
            box_lo: "+",
            box_lo_box_mid_fill: "-",
            box_mid: "+",
            box_mid_box_hi_fill: "-",
            box_hi: "+",
            box_hi_wh_hi_fill: " ",
            wh_hi: " ",
        },
    ],
};

static UNICODE_CHARS: BoxplotChars = BoxplotChars {
    marker: "✕",
    rows: [
        RowChars {
            wh_lo: "┬",
            wh_lo_box_lo_fill: " ",
            box_lo: "┌",
            box_lo_box_mid_fill: "─",
            box_mid: "┬",
            box_mid_box_hi_fill: "─",
            box_hi: "┐",
            box_hi_wh_hi_fill: " ",
            wh_hi: "┬",
        },
        RowChars {
            wh_lo: "├",
            wh_lo_box_lo_fill: "─",
            box_lo: "┤",
            box_lo_box_mid_fill: " ",
            box_mid: "│",
            box_mid_box_hi_fill: " ",
            box_hi: "├",
            box_hi_wh_hi_fill: "─",
            wh_hi: "┤",
        },
        RowChars {
            wh_lo: "┴",
            wh_lo_box_lo_fill: " ",
            box_lo: "└",
            box_lo_box_mid_fill: "─",
            box_mid: "┴",
            box_mid_box_hi_fill: "─",
            box_hi: "┘",
            box_hi_wh_hi_fill: " ",
            wh_hi: "┴",
        },
    ],
};

fn make_row(width: usize) -> Vec<String> {
    use std::iter::repeat;

    let mut row = vec![];
    row.extend(repeat(String::from("")).take(width));

    row
}

struct Plot(Vec<String>, Vec<String>, Vec<String>);

impl Plot {
    fn new(width: usize) -> Self {
        Plot(make_row(width), make_row(width), make_row(width))
    }

    fn render(&self) -> String {
        let rows = vec![
            self.0.join(""),
            self.1.join(""),
            self.2.join(""),
        ];

        rows.join("\n")
    }
}

pub fn summary_plot(summary: &Summary, width: usize, ascii: bool) -> String {
    let plot_style = if ascii { &ASCII_CHARS } else { &UNICODE_CHARS };

    plot_style.render(summary, width)
}

pub fn comparison_plot(
    summaries: &[&Summary],
    width: usize,
    ascii: bool,
    border: bool,
) -> String {
    if summaries.len() < 2 { unreachable!(); }

    let padding = if border { 2 } else { 0 };
    let border_style = if ascii {
        figure::ASCII_BORDER
    } else {
        figure::UNICODE_BORDER
    };
    use std::f64;
    let min = summaries.iter().map(|s| s.min()).fold(f64::MAX, |x, y| x.min(y));
    let max = summaries.iter().map(|s| s.max()).fold(f64::MIN, |x, y| x.max(y));
    let range = max - min;

    let mut plots = vec![];

    for s in summaries {
        let p = (s.max() - s.min()) / range;
        let w = (p * (width as f64)).floor() as usize - (padding * 2); // could underflow
        let plot = stamp::Stamp::new(&summary_plot(s, w, ascii)).unwrap();
        let left_padding = {
            let lpp = (s.min() - min) / range;
            (lpp * (width as f64)).floor() as usize
        } + padding;
        plots.push((plot, left_padding));
    }

    let height = &plots
        .iter()
        .map(|&(ref p, _)| p.height())
        .sum() + (padding * 2);

    let base = if border {
        figure::Border::new(border_style, width, height).render()
    } else {
        figure::Filled::blank(width, height).render()
    };

    let mut all_plots = stamp::Stamp::new(&base).unwrap();

    for (i, &(ref plot, left_padding)) in plots.iter().enumerate() {
        all_plots = all_plots.layer(&plot, left_padding, padding + i * plot.height()).unwrap();
    }

    all_plots.render()
}
