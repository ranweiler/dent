use summary::Summary;


pub struct TTest {
    pub p: f64,
    pub t: f64,
    pub df: f64,
}

pub fn t_test_2_sided(t: f64, df: f64) -> TTest {
    let p = 1.0 - t_cdf(t.abs(), df as f64);

    TTest { df, p, t }
}

pub fn welch_t_test(s1: &Summary, s2: &Summary) -> TTest {
    let (t, df) = welch_t_statistic(s1, s2);

    t_test_2_sided(t, df)
}

fn welch_t_statistic(s1: &Summary, s2: &Summary) -> (f64, f64) {
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

fn welch_satterthwaite_df(var1: f64, n1: f64, var2: f64, n2: f64) -> f64 {
    let df1 = n1 - 1.0;
    let df2 = n2 - 1.0;

    let num = ((var1 / n1) + (var2 / n2)).powi(2);
    let den = var1.powi(2) / (n1.powi(2) * df1) + var2.powi(2) / (n2.powi(2) * df2);
    let appx = num / den;

    appx
}

fn t_cdf(t: f64, v: f64) -> f64 {
    use num;

    let x = v / (v + t.powi(2));
    let a = 0.5 * v;
    let b = 0.5;

    1.0 - num::inc_beta(x, a, b).unwrap()
}
