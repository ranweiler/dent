#[derive(Debug)]
pub struct Summary {
    data: Vec<f64>,
}

impl Summary {
    /// Construct a `Summary` from a slice of 64-bit floating point numbers.
    ///
    /// This constructor is partial, and we obtain the following guarantees
    /// about the resulting sample data:
    ///
    ///   - The sample size is positive
    ///   - All values are finite
    ///   - The data are sorted
    ///
    pub fn new(data: &[f64]) -> Option<Self> {
        if data.len() == 0 {
            return None;
        }

        if data.iter().any(|x| !x.is_finite()) {
            return None;
        }

        let mut data = Vec::from(data);

        // Won't panic: we have checked that each float is finite.
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let s = Summary { data };

        Some(s)
    }

    pub fn as_slice(&self) -> &[f64] {
        self.data.as_slice()
    }

    pub fn size(&self) -> f64 {
        self.data.len() as f64
    }

    pub fn min(&self) -> f64 {
        self.data[0]
    }

    pub fn max(&self) -> f64 {
        self.data[self.data.len() - 1]
    }

    pub fn mean(&self) -> f64 {
        let t: f64 = self.data.iter().sum();

        t / self.size()
    }

    pub fn median(&self) -> f64 {
        let d = &self.data;
        let n = d.len();

        if n % 2 == 0 {
            (d[(n / 2) - 1] + d[n / 2]) / 2.0
        } else {
            d[(n - 1) / 2]
        }
    }

    /// Closest-ranks percentile computed via linear interpolation.
    /// See: http://www.itl.nist.gov/div898/handbook/prc/section2/prc262.htm
    pub fn percentile(&self, p: f64) -> Option<f64> {
        if !p.is_finite() { return None; }
        if p < 0.0 { return None; }
        if p >= 1.0 { return None; }

        let rank = (self.size() - 1.0) * p;
        let frac = rank.fract();

        let i = rank.floor() as usize;
        let j = i + 1;

        let xi = self.data[i];
        let xj = self.data[j];
        let x = xi + frac * (xj - xi);

        Some(x)
    }

    /// Uses Bessel's correction to estimate population variance.
    pub fn unbiased_variance(&self) -> f64 {
        let m = self.mean();
        let sum_sq_diff: f64 = self.data
            .iter()
            .map(|x| (x - m).powi(2))
            .sum();

        (1.0 / (self.size() - 1.0)) * sum_sq_diff
    }

    pub fn standard_deviation(&self) -> f64 {
        self.unbiased_variance().sqrt()
    }
}
