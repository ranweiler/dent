use summary::Summarizer;


pub struct LinearRegression {
    intercept: f64,
    r: f64,
    slope: f64,
    standard_error: f64,
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

    pub fn intercept(&self) -> f64 {
        self.intercept
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn slope(&self) -> f64 {
        self.slope
    }

    pub fn standard_error(&self) -> f64 {
        self.standard_error
    }

    fn simple_lr(x: &[f64], y: &[f64]) -> Self {
        let n = x.len() as f64;

        let summ_x = Summarizer::new(x).unwrap();
        let summ_y = Summarizer::new(y).unwrap();

        let mean_x = summ_x.mean();
        let mean_y = summ_y.mean();

        let std_x = summ_x.standard_deviation();
        let std_y = summ_y.standard_deviation();

        let r_num: f64 = (0..x.len())
            .map(|i| (x[i] - mean_x) * (y[i] - mean_y))
            .sum();
        let r_den = (n - 1.0) * std_x * std_y;
        let r = r_num / r_den;

        let slope = r * (std_y / std_x);
        let intercept = mean_y - slope * mean_x;

        let df = n - 2.0;
        let standard_error = (slope / df.sqrt()) * (1.0 / r.powi(2) - 1.0).sqrt();

        LinearRegression {
            intercept,
            r,
            slope,
            standard_error,
        }
    }
}
