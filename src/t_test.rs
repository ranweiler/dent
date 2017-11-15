use error::Error;
use summary::Summary;


/// The results and parameters of a two-sided, unequal-variances t-test.
pub struct TTest {
    pub p: f64,
    pub t: f64,
    pub df: f64,
}

pub fn t_test_2_sided(t: f64, df: f64) -> Result<TTest, Error> {
    let p = 1.0 - t_atv(t.abs(), df as f64)?;

    Ok(TTest { df, p, t })
}

/// Conduct a two-sided t-test that does not assume equal population variances.
pub fn welch_t_test(s1: &Summary, s2: &Summary) -> Result<TTest, Error> {
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

/// Degrees of freedom, approximated using the Welch-Satterthwaite equation [1].
///
/// [1]: http://www.itl.nist.gov/div898/handbook/mpc/section5/mpc571.htm
fn welch_satterthwaite_df(var1: f64, n1: f64, var2: f64, n2: f64) -> f64 {
    let df1 = n1 - 1.0;
    let df2 = n2 - 1.0;

    let num = ((var1 / n1) + (var2 / n2)).powi(2);
    let den = var1.powi(2) / (n1.powi(2) * df1) + var2.powi(2) / (n2.powi(2) * df2);
    let appx = num / den;

    appx
}

/// The definite integral of the density function of Student's t-distribution
/// over an interval [-t, t]. Also called the A(t|ν) function.
///
/// See equation 6.4.9 in [1].
///
/// [1]: "Numerical Recipes in C", 2nd Ed., p. 228
fn t_atv(t: f64, df: f64) -> Result<f64, Error> {
    use num;

    let x = df / (df + t.powi(2));
    let a = 0.5 * df;
    let b = 0.5;
    let ib = num::inc_beta(x, a, b)?;

    Ok(1.0 - ib)
}
