mod cmath {
    extern {
        pub fn lgamma(z: f64) -> f64;
    }
}

/// The natural logarithm of the gamma function [1].
///
/// [1]: https://www.encyclopediaofmath.org/index.php/Gamma-function
fn ln_gamma(z: f64) -> f64 {
    unsafe { cmath::lgamma(z) }
}

/// The complete beta function [1].
///
/// Computed using the equation [2] via the natural log-gamma function.
///
/// [1]: https://www.encyclopediaofmath.org/index.php/Beta-function
/// [2]: http://dlmf.nist.gov/8.17#E3
fn beta(a: f64, b: f64) -> f64 {
    (ln_gamma(a) + ln_gamma(b) - ln_gamma(a + b)).exp()
}

/// The regularized incomplete beta function [1].
///
/// We compute the incomplete beta function by using the modified Lentz's
/// algorithm [2] to evaluate its continued fraction representation [3].
/// Depending on the arguments, we use the symmetry relation [4] to guarantee a
/// bound that implies rapid convergence.
///
/// [1]: https://www.encyclopediaofmath.org/index.php/Incomplete_beta-function
/// [2]: "Numerical Recipes in C", 2nd Ed., p. 171
/// [3]: http://dlmf.nist.gov/8.17#E22
/// [4]: http://dlmf.nist.gov/8.17#E4
pub fn inc_beta(x: f64, a: f64, b: f64) -> Result<f64, ()> {
    if x < 0.0 { return Err(()); }
    if 1.0 < x { return Err(()); }

    let bound = (a + 1.0) / (a + b + 2.0);
    let ib = if x < bound {
        // The continued fraction will converge rapidly with the given args.

        // Leading coefficient of [3].
        let coeff = (x.powf(a) * (1.0 - x).powf(b))
            / (a * beta(a, b));

        coeff * inc_beta_cf(x, a, b)?
    } else {
        // Apply the identity `I_x(a, b) = 1 - I_{1-x}(b, a)` from [4].
        1.0 - inc_beta(1.0 - x, b, a)?
    };

    Ok(ib)
}

const INC_BETA_CF_APPX_ZERO: f64 = 1e-30;
const INC_BETA_CONVERGENCE_LIMIT: f64 = 1e-15;
const INC_BETA_MAX_ITER: usize = 1000;

/// This continued fraction part of the equation [1], evaluated using the
/// modified Lentz's algorithm.
///
/// [1]: http://dlmf.nist.gov/8.17#E22
fn inc_beta_cf(x: f64, a: f64, b: f64) -> Result<f64, ()> {
    let mut f = INC_BETA_CF_APPX_ZERO;
    let mut c = f;
    let mut d = 0.0;

    for i in 1..INC_BETA_MAX_ITER {
        let next = inc_beta_cf_step(x, a, b, i, f, c, d);
        let (_, _, _, del) = next;

        if (del - 1.0).abs() < INC_BETA_CONVERGENCE_LIMIT {
            return Ok(f);
        }

        f = next.0;
        c = next.1;
        d = next.2;
    }

    Err(())
}

/// Compute the next partial evaluation of the continued fraction, given the last.
fn inc_beta_cf_step(x: f64, a: f64, b: f64, i: usize, f0: f64, c0: f64, d0: f64) -> (f64, f64, f64, f64) {
    // The `i`th numerator of the continued fraction. This is given by the
    // sequence `(1, d_1, d_2, ...)`, with `d_i` as defined in [1].
    //
    // [1]: http://dlmf.nist.gov/8.17#E23
    let cf_num = if i == 1 {
        1.0
    } else {
        cf_d(i - 1, x, a, b)
    };

    let mut d = 1.0 + cf_num * d0;
    if d.abs() < INC_BETA_CF_APPX_ZERO {
        d = INC_BETA_CF_APPX_ZERO;
    }
    d = d.recip();

    let mut c = 1.0 + cf_num / c0;
    if c.abs() < INC_BETA_CF_APPX_ZERO {
        c = INC_BETA_CF_APPX_ZERO;
    }

    let del = c * d;
    let f = f0 * del;

    (f, c, d, del)
}

/// The sequence `d_i` given in [1] to define the terms of the continued
/// fraction. Note that it depends on `x`, `a`, and `b`.
///
/// [1]: http://dlmf.nist.gov/8.17#E23
fn cf_d(i: usize, x: f64, a: f64, b: f64) -> f64 {
    if i % 2 == 0 {
        let m = (i / 2) as f64;
        cf_d_even(m, x, a, b)
    } else {
        let m = ((i - 1) / 2) as f64;
        cf_d_odd(m, x, a, b)
    }
}

/// Even terms `d_i` with `i = 2m`.
fn cf_d_even(m: f64, x: f64, a: f64, b: f64) -> f64 {
    let num = m * (b - m) * x;
    let den = (a + 2.0 * m - 1.0) * (a + 2.0 * m);
    num / den
}

/// Odd terms `d_i` with `i = 2m + 1`.
fn cf_d_odd(m: f64, x: f64, a: f64, b: f64) -> f64 {
    let num = (a + m) * (a + b + m) * x;
    let den = (a + 2.0 * m) * (a + 2.0 * m + 1.0);
    -num / den
}
