use t_dist::{t_critical_value_2_sided};

pub use t_dist::SigLevel;

use summary::Summary;


pub struct TTest {
    pub t: f64,
    pub df: usize,
    pub alpha: f64,
    pub crit: f64,
    pub reject: bool,
}

pub fn t_test_2_sided(t: f64, df: usize, alpha: SigLevel) -> TTest {
    let crit = t_critical_value_2_sided(df, alpha);

    let reject = crit < t.abs();

    TTest {
        t,
        df,
        alpha: alpha.to_f64(),
        crit,
        reject,
    }
}

pub fn welch_t_test(s1: &Summary, s2: &Summary, alpha: SigLevel) -> TTest {
    let (t, df) = welch_t_statistic(s1, s2);

    t_test_2_sided(t, df, alpha)
}

fn welch_t_statistic(s1: &Summary, s2: &Summary) -> (f64, usize) {
    let n1 = s1.size();
    let m1 = s1.mean();
    let var1 = s1.unbiased_variance();

    let n2 = s2.size();
    let m2 = s2.mean();
    let var2 = s2.unbiased_variance();

    let s_delta_bar = ((var1 / n1) + (var2 / n2)).sqrt();
    let t = (m1 - m2) / s_delta_bar;

    let df = welch_satterthwaite_df(var1, n1, var2, n2);

    (t, df)
}

fn welch_satterthwaite_df(var1: f64, n1: f64, var2: f64, n2: f64) -> usize {
    let df1 = n1 - 1.0;
    let df2 = n2 - 1.0;

    let num = ((var1 / n1) + (var2 / n2)).powi(2);
    let den = var1.powi(2) / (n1.powi(2) * df1) + var2.powi(2) / (n2.powi(2) * df2);
    let appx = num / den;

    appx.round() as usize
}
