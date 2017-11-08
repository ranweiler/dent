mod figure;

use stamp;

use summary::Summary;


macro_rules! plot {
    ($p: expr) => {
        match $p {
            Ok(t) => Ok(t),
            Err(_) => Err("Unable to plot sample data"),
        }
    }
}

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
        let range = summary.range();
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

    fn from_summary_no_outliers(summary: &Summary) -> Self {
        let min = summary.min_non_outlier().min(summary.mean());
        let max = summary.max_non_outlier().max(summary.mean());
        let range = max - min;
        let n = |x| (x - min) / range;

        Boxplot {
            box_lo: n(summary.lower_quartile()),
            box_mid: n(summary.median()),
            box_hi: n(summary.upper_quartile()),
            marker: n(summary.mean()),
            wh_lo: n(summary.min_non_outlier()),
            wh_hi: n(summary.max_non_outlier()),
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
        let max_col = (width - 1) as f64;
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
        // Lower whisker extent.
        for i in (cols.wh_lo + 1)..cols.box_lo {
            row[i] = self.wh_lo_box_lo_fill.to_string();
        }

        // Upper whisker extent.
        for i in (cols.box_hi + 1)..cols.wh_hi {
            row[i] = self.box_hi_wh_hi_fill.to_string();
        }

        // Lower box extent.
        for i in (cols.box_lo + 1)..cols.box_mid {
            row[i] = self.box_lo_box_mid_fill.to_string();
        }

        // Upper box extent.
        for i in (cols.box_mid + 1)..cols.box_hi {
            row[i] = self.box_mid_box_hi_fill.to_string();
        }

        // Lower box end.
        row[cols.box_lo] = self.box_lo.to_string();

        // Upper box end.
        row[cols.box_hi] = self.box_hi.to_string();

        // Lower whisker end.
        row[cols.wh_lo] = self.wh_lo.to_string();

        // Upper whisker end.
        row[cols.wh_hi] = self.wh_hi.to_string();

        // Middle line.
        row[cols.box_mid] = self.box_mid.to_string();
    }
}

struct BoxplotChars {
    marker: &'static str,
    rows: [RowChars; 3],
}

impl BoxplotChars {
    pub fn render(&self, summary: &Summary, width: usize, outliers: bool)
                  -> Result<String, &'static str> {
        let data = if outliers {
            Boxplot::from_summary(summary)
        } else {
            Boxplot::from_summary_no_outliers(summary)
        };
        let cols = BoxplotCols::new(&data, width);
        let mut plot = Plot::new(width);

        self.rows[0].render(&mut plot.0, &cols);
        self.rows[1].render(&mut plot.1, &cols);
        self.rows[2].render(&mut plot.2, &cols);

        let no_marker = plot.render();

        let base = plot!(stamp::Stamp::new(&no_marker))?;
        let marker = plot!(stamp::Stamp::new(self.marker))?;
        let layered = plot!(base.layer(&marker, cols.marker, 1))?;

        Ok(layered.render())
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

pub fn summary_plot(summary: &Summary, width: usize, ascii: bool, outliers: bool)
                    -> Result<String, &'static str> {
    let plot_style = if ascii { &ASCII_CHARS } else { &UNICODE_CHARS };

    plot_style.render(summary, width, outliers)
}

pub fn comparison_plot(
    summaries: &[&Summary],
    width: usize,
    ascii: bool,
    border: bool,
    outliers: bool,
) -> Result<String, &'static str> {
    if summaries.is_empty() {
        return Err("Cannot plot empty list of summaries");
    }

    let padding = if border { 2 } else { 0 };
    let content_width = (width - 2 * padding) as f64;
    let border_style = if ascii {
        figure::ASCII_BORDER
    } else {
        figure::UNICODE_BORDER
    };

    use std::f64;

    let plot_min = |s: &Summary| if outliers {
        s.min()
    } else {
        s.min_non_outlier().min(s.mean())
    };
    let min = summaries
        .iter()
        .map(|s| plot_min(s))
        .fold(f64::MAX, |x, y| x.min(y));

    let plot_max = |s: &Summary| if outliers {
        s.max()
    } else {
        s.max_non_outlier().max(s.mean())
    };
    let max = summaries
        .iter()
        .map(|s| plot_max(s))
        .fold(f64::MIN, |x, y| x.max(y));

    // Used to compute relative widths of boxplots from their own ranges.
    let range = max - min;

    let mut plots = vec![];

    for s in summaries {
        let s_min = if outliers {
            s.min()
        } else {
            s.min_non_outlier().min(s.mean())
        };
        let s_max = if outliers {
            s.max()
        } else {
            s.max_non_outlier().max(s.mean())
        };

        // Proportion of total content width spanned by this plot.
        let p = (s_max - s_min) / range;

        // Boxplot content width in cols.
        let w = (content_width * p).floor().max(1.0);
        assert!(1.0 <= w);
        assert!(w <= content_width);

        let plot = plot!(stamp::Stamp::new(&summary_plot(s, w as usize, ascii, outliers)?))?;

        assert!(min <= s_min);
        let offset_p = (s_min - min) / range;

        let offset = (offset_p * content_width).min(content_width - w);
        assert!(offset + w <= content_width);

        plots.push((plot, padding + (offset as usize)));
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

    let mut all_plots = plot!(stamp::Stamp::new(&base))?;

    for (i, &(ref plot, left_offset)) in plots.iter().enumerate() {
        all_plots = plot!(all_plots.layer(&plot, left_offset, padding + i * plot.height()))?;
    }

    Ok(all_plots.render())
}
