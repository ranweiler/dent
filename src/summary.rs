use error::Error;


#[derive(Debug)]
pub struct Summarizer {
    data: Vec<f64>,
}

impl Summarizer {
    /// Construct a `Summarizer` from a slice of 64-bit floating point numbers.
    ///
    /// This constructor is partial, and we obtain the following guarantees
    /// about the resulting sample data:
    ///
    ///   - The sample size is positive
    ///   - All values are finite
    ///   - The data are sorted
    ///
    pub fn new(data: &[f64]) -> Result<Self, Error> {
        if data.is_empty() {
            return Err(Error::EmptySample);
        }

        if data.iter().any(|x| !x.is_finite()) {
            return Err(Error::BadSample);
        }

        let mut data = Vec::from(data);

        // Won't panic: we have checked that each float is finite.
        data.sort_by(|a, b| a.partial_cmp(b).unwrap_or_else(|| unreachable!()));

        let s = Summarizer { data };

        Ok(s)
    }

    pub fn as_slice(&self) -> &[f64] {
        self.data.as_slice()
    }

    pub fn size(&self) -> f64 {
        self.data.len() as f64
    }

    pub fn iqr(&self) -> f64 {
        self.upper_quartile() - self.lower_quartile()
    }

    pub fn lower_quartile(&self) -> f64 {
        // Statically known to be defined.
        self.percentile(0.25).unwrap_or_else(|_| unreachable!())
    }

    pub fn min(&self) -> f64 {
        self.data[0]
    }

    pub fn min_non_outlier(&self) -> f64 {
        let lower_outlier_bound = self.lower_quartile() - 1.5 * self.iqr();

        self.data
            .iter()
            .cloned()
            .find(|&x| lower_outlier_bound <= x)
            .unwrap_or_else(|| unreachable!())  // By definition of quartile.
    }

    pub fn max(&self) -> f64 {
        self.data[self.data.len() - 1]
    }

    pub fn max_non_outlier(&self) -> f64 {
        let upper_outlier_bound = self.upper_quartile() + 1.5 * self.iqr();

        self.data
            .iter()
            .cloned()
            .rev()
            .find(|&x| x <= upper_outlier_bound)
            .unwrap_or_else(|| unreachable!())  // By definition of quartile.
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
    ///
    /// According to NIST, there isn't a standard computational definition of percentile.
    /// We take a practical approach that aims to be both unsurprising and consistent with
    /// common statistics packages. In particular, our implementation guarantees that the
    /// boundary percentiles correspond to the sample min and max.
    pub fn percentile(&self, p: f64) -> Result<f64, Error> {
        if !p.is_finite() { return Err(Error::Undefined); }
        if p < 0.0 || 1.0 < p {
            return Err(Error::Undefined);
        }

        let rank = (self.size() - 1.0) * p;
        let frac = rank.fract();

        let i = rank.floor() as usize;
        let j = i + 1;

        if j == self.data.len() {
            // This implies that `i` indexes the largest data point in the sample.
            // Dereferencing at `j` would be an error, but `i` is exactly the max.
            return Ok(self.data[i]);
        }

        let xi = self.data[i];
        let xj = self.data[j];
        let x = xi + frac * (xj - xi);

        Ok(x)
    }

    pub fn range(&self) -> f64 {
        self.max() - self.min()
    }

    pub fn upper_quartile(&self) -> f64 {
        // Statically known to be defined.
        self.percentile(0.75).unwrap_or_else(|_| unreachable!())
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

    pub fn standard_error(&self) -> f64 {
        self.standard_deviation() / self.size().sqrt()
    }
}

#[derive(Debug)]
pub struct Summary {
    iqr: f64,
    len: usize,
    lower_quartile: f64,
    min: f64,
    min_non_outlier: f64,
    max: f64,
    max_non_outlier: f64,
    mean: f64,
    median: f64,
    range: f64,
    standard_deviation: f64,
    standard_error: f64,
    unbiased_variance: f64,
    upper_quartile: f64,
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
    pub fn new(data: &[f64]) -> Result<Self, Error> {
        let s = Summarizer::new(data)?;

        Ok(Summary {
            iqr: s.iqr(),
            len: s.data.len(),
            lower_quartile: s.lower_quartile(),
            min: s.min(),
            min_non_outlier: s.min_non_outlier(),
            max: s.max(),
            max_non_outlier: s.max_non_outlier(),
            mean: s.mean(),
            median: s.median(),
            range: s.range(),
            upper_quartile: s.upper_quartile(),
            unbiased_variance: s.unbiased_variance(),
            standard_deviation: s.standard_deviation(),
            standard_error: s.standard_error(),
        })
    }

    pub fn size(&self) -> f64 {
        self.len as f64
    }

    pub fn range(&self) -> f64 {
        self.range
    }

    pub fn iqr(&self) -> f64 {
        self.iqr
    }

    pub fn lower_quartile(&self) -> f64 {
        self.lower_quartile
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn min_non_outlier(&self) -> f64 {
        self.min_non_outlier
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn max_non_outlier(&self) -> f64 {
        self.max_non_outlier
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn median(&self) -> f64 {
        self.median
    }

    /// Uses Bessel's correction to estimate population variance.
    pub fn unbiased_variance(&self) -> f64 {
        self.unbiased_variance
    }

    pub fn upper_quartile(&self) -> f64 {
        self.upper_quartile
    }

    pub fn standard_deviation(&self) -> f64 {
        self.standard_deviation
    }

    pub fn standard_error(&self) -> f64 {
        self.standard_error
    }
}
