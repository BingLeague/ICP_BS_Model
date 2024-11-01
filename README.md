# ICP_BS_Model
bs model in Rust, on ICP chain


Here's an introduction to the Black-Scholes (BS) model suitable for a README file:

---

## Black-Scholes Model Overview

The Black-Scholes model, developed by economists Fischer Black and Myron Scholes, is one of the most widely used mathematical models for pricing European-style options. It provides a formula that calculates the theoretical price of call and put options based on key financial variables, including the current price of the underlying asset, the option's strike price, time to expiration, risk-free interest rate, and asset volatility.

### Formula

For a European call option, the Black-Scholes formula is defined as:

\[ C = S \cdot N(d1) - K \cdot e^{-rt} \cdot N(d2) \]

Where:
- \( S \): Current asset price
- \( K \): Strike price of the option
- \( r \): Risk-free interest rate
- \( t \): Time until expiration (in years)
- \( \sigma \): Volatility of the asset
- \( d1 = \frac{\ln(\frac{S}{K}) + (r + \frac{\sigma^2}{2}) \cdot t}{\sigma \cdot \sqrt{t}} \)
- \( d2 = d1 - \sigma \cdot \sqrt{t} \)
- \( N \): Cumulative distribution function (CDF) for a standard normal distribution

The model provides a foundation for understanding how the option’s price changes in response to different inputs and is widely used for risk management and in financial derivatives markets.

### Implementation

This repository provides a Rust implementation of the Black-Scholes model, designed to calculate the prices of European call and put options. The code includes a custom approximation of the cumulative distribution function (CDF) to ensure compatibility with environments, like the Internet Computer (ICP) blockchain, where standard libraries may be limited.

