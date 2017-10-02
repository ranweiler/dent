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
    summary1: &Summary,
    summary2: &Summary,
    width: usize,
    ascii: bool,
    border: bool,
) -> String {
    let padding = if border { 2 } else { 0 };
    let border_style = if ascii {
        figure::ASCII_BORDER
    } else {
        figure::UNICODE_BORDER
    };

    let min = summary1.min().min(summary2.min());
    let max = summary1.max().max(summary2.max());

    let range = max - min;
    let range1 = summary1.max() - summary1.min();
    let range2 = summary2.max() - summary2.min();

    let proportion1 = range1 / range;
    let proportion2 = range2 / range;

    let width1 = (proportion1 * (width as f64)).floor() as usize - (padding * 2);
    let width2 = (proportion2 * (width as f64)).floor() as usize - (padding * 2);

    let plot1 = stamp::Stamp::new(&summary_plot(&summary1, width1, ascii)).unwrap();
    let plot2 = stamp::Stamp::new(&summary_plot(&summary2, width2, ascii)).unwrap();

    let offset1 = {
        let neg_proportion = (summary1.min() - min) / range;
        (neg_proportion * (width as f64)).floor() as usize
    } + padding;
    let offset2 = {
        let neg_proportion = (summary2.min() - min) / range;
        (neg_proportion * (width as f64)).floor() as usize
    } + padding;

    let height = plot1.height() + plot2.height() + (padding * 2);

    let base = if border {
        figure::Border::new(border_style, width, height).render()
    } else {
        figure::Filled::blank(width, height).render()
    };

    let mut plots = stamp::Stamp::new(&base).unwrap();
    plots = plots.layer(&plot1, offset1, padding).unwrap();
    plots = plots.layer(&plot2, offset2, plot1.height() + padding).unwrap();

    plots.render()
}
