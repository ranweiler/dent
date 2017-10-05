use summary::Summarizer;


pub struct LinearRegression {
    pub intercept: f64,
    pub r: f64,
    pub slope: f64,
}

impl LinearRegression {
    pub fn new(ind: &[f64], dep: &[f64]) -> Result<Self, ()> {
        if ind.len() != dep.len() {
            return Err(());
        }

        if ind.len() == 0 {
            return Err(());
        }

        Ok(LinearRegression::simple_lr(ind, dep))
    }

    fn simple_lr(x: &[f64], y: &[f64]) -> Self {
        let n = x.len();

        let summ_x = Summarizer::new(x).unwrap();
        let summ_y = Summarizer::new(y).unwrap();

        let mean_x = summ_x.mean();
        let mean_y = summ_y.mean();

        let std_x = summ_x.standard_deviation();
        let std_y = summ_y.standard_deviation();

        let r_num: f64 = (0..n).map(|i| (x[i] - mean_x) * (y[i] - mean_y)).sum();
        let r_den = (n as f64 - 1.0) * std_x * std_y;
        let r = r_num / r_den;

        let slope = r * (std_y / std_x);
        let intercept = mean_y - slope * mean_x;

        LinearRegression {
            intercept,
            r,
            slope,
        }
    }
}
