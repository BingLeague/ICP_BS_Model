use std::f64::consts::PI;

/// Implements the error function (erf), which is essential for calculating the CDF of a standard normal distribution
/// Using approximation for faster calculation, as exact computation is complex and not needed here
fn erf(x: f64) -> f64 {
    // Coefficients for approximation (Abramowitz and Stegun approximation)
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;

    // Save the sign of x
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    // A&S formula 7.1.26
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

/// Manually computes the cumulative distribution function (CDF) for a standard normal distribution
/// using the error function (erf).
fn norm_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / (2.0f64).sqrt()))
}

/// Computes the price of a European call option (Black-Scholes model)
///
/// # Parameters
/// * `s` - The asset's current price
/// * `k` - Strike price of the option
/// * `r` - Risk-free interest rate
/// * `t` - Time until expiration (in years)
/// * `sigma` - Volatility of the asset
///
/// # Returns
/// The price of a European call option
pub fn call_option_price(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    let d1 = (s.ln() + (r + 0.5 * sigma.powi(2)) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    s * norm_cdf(d1) - k * (-r * t).exp() * norm_cdf(d2)
}

/// Computes the price of a European put option (Black-Scholes model)
///
/// # Parameters
/// * `s` - The asset's current price
/// * `k` - Strike price of the option
/// * `r` - Risk-free interest rate
/// * `t` - Time until expiration (in years)
/// * `sigma` - Volatility of the asset
///
/// # Returns
/// The price of a European put option
pub fn put_option_price(s: f64, k: f64, r: f64, t: f64, sigma: f64) -> f64 {
    let d1 = (s.ln() + (r + 0.5 * sigma.powi(2)) * t) / (sigma * t.sqrt());
    let d2 = d1 - sigma * t.sqrt();

    k * (-r * t).exp() * norm_cdf(-d2) - s * norm_cdf(-d1)
}

// Example usage
fn main() {
    let s = 100.0;      // Current price of the asset
    let k = 100.0;      // Strike price of the option
    let r = 0.05;       // Risk-free interest rate
    let t = 1.0;        // Time to expiration in years
    let sigma = 0.2;    // Volatility of the asset

    let call_price = call_option_price(s, k, r, t, sigma);
    let put_price = put_option_price(s, k, r, t, sigma);

    println!("European Call Option Price: {}", call_price);
    println!("European Put Option Price: {}", put_price);
}
