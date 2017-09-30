use summary::Summary;

struct BoxWhiskerData {
    box_lo: f64,
    box_mid: f64,
    box_hi: f64,
    wh_lo: f64,
    wh_hi: f64,
}

impl BoxWhiskerData {
    fn from_summary(summary: &Summary) -> Self {
        BoxWhiskerData {
            box_lo: summary.percentile(0.25).unwrap(),
            box_mid: summary.median(),
            box_hi: summary.percentile(0.75).unwrap(),
            wh_lo: summary.min(),
            wh_hi: summary.max(),
        }
    }
}

struct BoxWhiskerCols {
    box_lo: usize,
    box_mid: usize,
    box_hi: usize,
    wh_lo: usize,
    wh_hi: usize,
}

impl BoxWhiskerCols {
    fn new(data: &BoxWhiskerData, width: usize) -> Self {
        let max_col = width - 1;
        let range = data.wh_hi - data.wh_lo;
        let normalize = |x| (x - data.wh_lo) / range;
        let scale = |x| (normalize(x) * (max_col as f64)) as f64;
        let to_col = |x| scale(x).floor() as usize;

        BoxWhiskerCols {
            box_lo: to_col(data.box_lo),
            box_mid: to_col(data.box_mid),
            box_hi: to_col(data.box_hi),
            wh_lo: to_col(data.wh_lo),
            wh_hi: to_col(data.wh_hi),
        }
    }
}

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

struct RowChars {
    wh_lo: String,
    wh_lo_box_lo_fill: String,
    box_lo: String,
    box_lo_box_mid_fill: String,
    box_mid: String,
    box_mid_box_hi_fill: String,
    box_hi: String,
    box_hi_wh_hi_fill: String,
    wh_hi: String,
}

impl RowChars {
    fn render(&self, row: &mut Vec<String>, cols: &BoxWhiskerCols) {
        row[cols.wh_lo] = self.wh_lo.clone();
        for i in (cols.wh_lo + 1)..cols.box_lo {
            row[i] = self.wh_lo_box_lo_fill.clone();
        }
        row[cols.box_lo] = self.box_lo.clone();
        for i in (cols.box_lo + 1)..cols.box_mid {
            row[i] = self.box_lo_box_mid_fill.clone();
        }
        row[cols.box_mid] = self.box_mid.clone();
        for i in (cols.box_mid + 1)..cols.box_hi {
            row[i] = self.box_mid_box_hi_fill.clone();
        }
        row[cols.box_hi] = self.box_hi.clone();
        for i in (cols.box_hi + 1)..cols.wh_hi {
            row[i] = self.box_hi_wh_hi_fill.clone();
        }
        row[cols.wh_hi] = self.wh_hi.clone();
    }
}

pub fn summary_plot(summary: &Summary, width: usize) -> String {
    let data = BoxWhiskerData::from_summary(summary);
    let cols = BoxWhiskerCols::new(&data, width);

    let mut plot = Plot::new(width);

    let chars1 = RowChars {
        wh_lo: String::from("┬"),
        wh_lo_box_lo_fill: String::from(" "),
        box_lo: String::from("┌"),
        box_lo_box_mid_fill: String::from("─"),
        box_mid: String::from("┬"),
        box_mid_box_hi_fill: String::from("─"),
        box_hi: String::from("┐"),
        box_hi_wh_hi_fill: String::from(" "),
        wh_hi: String::from("┬"),
    };
    chars1.render(&mut plot.0, &cols);

    let chars2 = RowChars {
        wh_lo: String::from("├"),
        wh_lo_box_lo_fill: String::from("─"),
        box_lo: String::from("┤"),
        box_lo_box_mid_fill: String::from(" "),
        box_mid: String::from("│"),
        box_mid_box_hi_fill: String::from(" "),
        box_hi: String::from("├"),
        box_hi_wh_hi_fill: String::from("─"),
        wh_hi: String::from("┤"),
    };
    chars2.render(&mut plot.1, &cols);

    let chars3 = RowChars {
        wh_lo: String::from("┴"),
        wh_lo_box_lo_fill: String::from(" "),
        box_lo: String::from("└"),
        box_lo_box_mid_fill: String::from("─"),
        box_mid: String::from("┴"),
        box_mid_box_hi_fill: String::from("─"),
        box_hi: String::from("┘"),
        box_hi_wh_hi_fill: String::from(" "),
        wh_hi: String::from("┴"),
    };
    chars3.render(&mut plot.2, &cols);

    plot.render()
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

    let plot1 = summary_plot(&summary1, width1);
    let plot2 = summary_plot(&summary2, width2);

    format!("{}\n{}", pad(&plot1, offset1), pad(&plot2, offset2))
}
