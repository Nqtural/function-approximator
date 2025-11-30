# Function Approximator

A fully from-scratch neural network written in Rust that learns to approximate mathematical functions (e.g. sin(x)).

---

## Overview

This project implements a minimal neural network core from scratch in Rust — no ML frameworks, no ndarray, no external math libraries.
It demonstrates:
- Manual matrix multiplication
- Manual forward pass
- Manual backpropagation
- Tanh activation (custom implementation)
- Training on randomly sampled inputs
- Terminal-based visualization of predictions vs. real values

The network is used to approximate any single-input / single-output function (sin(x) by default).

## License

Licensed under [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html).
