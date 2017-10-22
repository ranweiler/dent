fn exp_parts(x: f64) -> (String, String) {
    let s = format!("{:e}", x);
    let parts: Vec<_> = s.split("e").collect();
    let c = parts[0].to_string();
    let e = parts[1].to_string();

    (c, e)
}

/// Try to format a float `x` such that the resulting string length is at most
/// `max_len`. The output may or may not be in `std::fmt::LowerExp` scientific
/// notation. The goal is to produce a human-readable string, permitting lost
/// precision. This function should not be used to produce output that must
/// round-trip, or is meant for machine consumption.
pub fn f(x: f64, max_len: usize) -> String {
    // We expect to be able to approximately represent any finite float in 6
    // characters, with a limiting example being `-std::f64::MIN_POSITIVE`
    // (2.2250738585072014e-308), which can be approximated as "-2e-308".
    if max_len < 6 {
        panic!("Max output length must be at least 6");
    }

    // Check the output of the default `Display` formatter. If it meets our
    // length bound, use it, since we are sure it is short and well-formatted.
    let s = format!("{}", x);

    if s.len() <= max_len {
        return s;
    }
    // If we are here, the default `Display` formatter produced a result that
    // was too long for us. Note that this implies that `x` != 0.

    // Check the exponent of the normalized scientific notation form of the
    // number. If it is 0, then we don't want to use scientific notation, since
    // we'd waste 2 characters on the suffix "e0".
    //
    // If the exponent is -1, then in scientific notation, our output would have
    // both a decimal point "." and suffix "e-1", a total of 4 insignificant
    // characters. In a fixed-precision encoding, we would have a "0." prefix,
    // but the rest of the string would contain significant digits (only 2
    // insignificant characters), so we prefer that.
    let (_, e) = exp_parts(x);
    let use_exp = (&e != "0") && (&e != "-1");

    // Count precisions `p` down from `max_len` - 1. At each step, check the
    // fixed-precision string encoding of `x`, using the format style we
    // determined above. Stop when we find the longest (most precise) encoding
    // that meets our total length bound. We can skip the case `p` == `max_len`,
    // because if we are here, our output will include a decimal point, so that
    // case will never meet our length bound anyway.
    for p in (1..max_len).rev() {
        let s = if use_exp {
            format!("{x:.p$e}", p = p, x = x)
        } else {
            format!("{x:.p$}", p = p, x = x)
        };

        if s.len() <= max_len {
            return s;
        }
    }

    format!("{:.0e}", x)
}
