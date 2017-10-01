use summary::Summary;

struct BoxplotData {
    box_lo: f64,
    box_mid: f64,
    box_hi: f64,
    wh_lo: f64,
    wh_hi: f64,
}

impl BoxplotData {
    fn from_summary(summary: &Summary) -> Self {
        BoxplotData {
            box_lo: summary.percentile(0.25).unwrap(),
            box_mid: summary.median(),
            box_hi: summary.percentile(0.75).unwrap(),
            wh_lo: summary.min(),
            wh_hi: summary.max(),
        }
    }
}

struct BoxplotCols {
    box_lo: usize,
    box_mid: usize,
    box_hi: usize,
    wh_lo: usize,
    wh_hi: usize,
}

impl BoxplotCols {
    fn new(data: &BoxplotData, width: usize) -> Self {
        let max_col = width - 1;
        let range = data.wh_hi - data.wh_lo;
        let normalize = |x| (x - data.wh_lo) / range;
        let scale = |x| (normalize(x) * (max_col as f64)) as f64;
        let to_col = |x| scale(x).floor() as usize;

        BoxplotCols {
            box_lo: to_col(data.box_lo),
            box_mid: to_col(data.box_mid),
            box_hi: to_col(data.box_hi),
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

struct BoxplotChars(RowChars, RowChars, RowChars);

impl BoxplotChars {
    pub fn plot(&self, summary: &Summary, width: usize) -> String {
        let data = BoxplotData::from_summary(summary);
        let cols = BoxplotCols::new(&data, width);

        let mut plot = Plot::new(width);

        self.0.render(&mut plot.0, &cols);
        self.1.render(&mut plot.1, &cols);
        self.2.render(&mut plot.2, &cols);

        plot.render()
    }
}

static ASCII_CHARS: BoxplotChars = BoxplotChars(
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
);

static UNICODE_CHARS: BoxplotChars = BoxplotChars(
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
);

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

pub fn ascii_summary_plot(summary: &Summary, width: usize) -> String {
    ASCII_CHARS.plot(summary, width)
}

pub fn summary_plot(summary: &Summary, width: usize) -> String {
    UNICODE_CHARS.plot(summary, width)
}

fn make_padding(cols: usize) -> String {
    use std::iter::{FromIterator, repeat};

    String::from_iter(repeat(String::from(" ")).take(cols))
}

fn pad(s: &String, cols: usize) -> String {
    let padding = make_padding(cols);

    let padded: Vec<String> = s
        .split("\n")
        .map(|l| format!("{}{}", padding, l))
        .collect();

    padded.join("\n")
}

pub fn comparison_plot(
    summary1: &Summary,
    summary2: &Summary,
    width: usize,
    ascii: bool,
) -> String {
    let min = summary1.min().min(summary2.min());
    let max = summary1.max().max(summary2.max());

    let range = max - min;
    let range1 = summary1.max() - summary1.min();
    let range2 = summary2.max() - summary2.min();

    let proportion1 = range1 / range;
    let proportion2 = range2 / range;

    let width1 = (proportion1 * (width as f64)).floor() as usize;
    let width2 = (proportion2 * (width as f64)).floor() as usize;

    let offset1 = {
        let neg_proportion = (summary1.min() - min) / range;
        (neg_proportion * (width as f64)).floor() as usize
    };
    let offset2 = {
        let neg_proportion = (summary2.min() - min) / range;
        (neg_proportion * (width as f64)).floor() as usize
    };

    let (plot1, plot2) = if ascii {
        let plot1 = ascii_summary_plot(&summary1, width1);
        let plot2 = ascii_summary_plot(&summary2, width2);
        (plot1, plot2)
    } else {
        let plot1 = summary_plot(&summary1, width1);
        let plot2 = summary_plot(&summary2, width2);
        (plot1, plot2)
    };

    format!("{}\n{}", pad(&plot1, offset1), pad(&plot2, offset2))
}
