# ICP_BS_Model， bs model in Rust, on ICP chain

---

## Black-Scholes Model Overview

The Black-Scholes model, developed by economists Fischer Black and Myron Scholes, is one of the most widely used mathematical models for pricing European-style options. It provides a formula that calculates the theoretical price of call and put options based on key financial variables, including the current price of the underlying asset, the option's strike price, time to expiration, risk-free interest rate, and asset volatility.

### Formula

<p align="center">
  <img width="800" src="/formula.png">
</p>


The model provides a foundation for understanding how the option’s price changes in response to different inputs and is widely used for risk management and in financial derivatives markets.

### Implementation

This repository provides a Rust implementation of the Black-Scholes model, designed to calculate the prices of European call and put options. The code includes a custom approximation of the cumulative distribution function (CDF) to ensure compatibility with environments, like the Internet Computer (ICP) blockchain, where standard libraries may be limited.

