use error::Error;
use summary::Summarizer;


/// The results of a simple linear regression with one predictor variable and
/// one response variable.
pub struct LinearRegression {
    intercept: f64,
    r: f64,
    slope: f64,
    standard_error: f64,
}

impl LinearRegression {
    /// Fit the sample data to a linear model `Y = αX + β`, where `X` is the
    /// predictor variable and `Y` is the response variable.
    ///
    /// The sample data points are pairs of the form `(x, y)`, where each `x` is
    /// interpreted as an observed value of the predictor variable and `y` is a
    /// value of the response variable.
    pub fn new(data: &[(f64, f64)]) -> Result<Self, Error> {
        if data.is_empty() {
            return Err(Error::EmptySample);
        }

        LinearRegression::simple_lr(data)
    }

    /// Intercept `β` of the fitted linear model `Y = αX + β`.
    pub fn intercept(&self) -> f64 {
        self.intercept
    }

    /// Pearson's correlation coefficient.
    pub fn r(&self) -> f64 {
        self.r
    }

    /// Slope coefficient `α` of the fitted linear model `Y = αX + β`.
    pub fn slope(&self) -> f64 {
        self.slope
    }

    /// Standard error of the estimate.
    pub fn standard_error(&self) -> f64 {
        self.standard_error
    }

    fn simple_lr(data: &[(f64, f64)]) -> Result<Self, Error> {
        let n = data.len() as f64;

        let (x, y): (Vec<_,>, Vec<_>) = data.iter().cloned().unzip();

        let summ_x = Summarizer::new(&x)?;
        let summ_y = Summarizer::new(&y)?;

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

        Ok(LinearRegression {
            intercept,
            r,
            slope,
            standard_error,
        })
    }
}
