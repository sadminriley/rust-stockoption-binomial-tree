extern crate ndarray;

use ndarray::Array2;

/*
    I'm not going to get into the actual weeds of real Financial Quantitative Analysis or the general "what/why" - but in short
    form:
    Binomials are one of many mathematical models being used to price certain securities; based on
    derivative financial instruments. This can also be referred to in plain investor English
    as Delta Hedging(...aka just good old fashioned Arbitrage for any breathing Finance heads out there, aka Continuous Delta Hedge formula, and
   aka...Yeah actually thats it! Super easy to remember! /s)

    https://en.wikipedia.org/wiki/Delta_neutral

    https://en.wikipedia.org/wiki/Binomial_options_pricing_model
    This is the simplest way I could come up with doing this in Rust. Feel free to submit any suggestions!
    Still awake?! Ok thanks!
*/

// While the above can be examples of how the actual formula is written, let's simplify it. Please remember this is a WIP.
pub fn calculate_binomials(
    num_iterations: usize,
    price: f64,
    change: f64,
    risk: f64,
    strike: f64,
) -> f64 {
    let n: usize = num_iterations; // Use the provided number of iterations
    let s0: f64 = price; // Provided price
    let u: f64 = change; // Provided change value. aka Upward. This let's us calculate for upward/downward price movement
    let r: f64 = risk; // Risk value
    let k: f64 = strike; // Strike price

    let d: f64 = 1.0 / u; // Downward.
    let p: f64 = (1.0 + r - d) / (u - d);
    let q: f64 = 1.0 - p;

    // Create stock price tree
    let mut stock = Array2::<f64>::zeros((n + 1, n + 1));
    for i in 0..=n {
        for j in 0..=i {
            stock[[j, i]] = s0 * u.powi((i - j) as i32) * d.powi(j as i32);
        }
    }

    println!("Stock Tree: \n{:?}", stock);

    // Create option price tree. Calculate the options price based on N and work backwards.
    let mut option = Array2::<f64>::zeros((n + 1, n + 1));
    for j in 0..=n {
        option[[j, n]] = (stock[[j, n]] - k).max(0.0);
    }

    // Generate option prices recursively
    for i in (0..n).rev() {
        for j in 0..=i {
            option[[j, i]] =
                (1.0 / (1.0 + r)) * (p * option[[j, i + 1]] + q * option[[j + 1, i + 1]]);
        }
    }

    println!("Option Tree: \n{:?}", option);

    // Finally return the option price at the end of the tree
    option[[0, 0]]
}

fn main() {
    // Calculate it!
    let result: f64 = calculate_binomials(1, 23.0, 1.2, 1.23, 5.00); // These obviously don't reflect any position and are mock

    // let result: f64 = calculate_binomials(num_iterations, price, change, risk, strike)
    println!("Result: {}", result);
}
