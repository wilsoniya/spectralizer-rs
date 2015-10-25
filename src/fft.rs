//! Funcionality related to computing Fourier transforms.

use num::Complex;
use std::f64::consts::PI;

/// Real-valued FFT. Essentially just ignores imaginary components of
/// Cooley-Tukey.
pub fn real_fft(x: &[f64], X: &mut [f64]) {
    assert!(x.len() == X.len());
    let x_complex: Vec<Complex<f64>> = x.iter().map(|&n| Complex::new(n, 0.0)).collect();
    let ret_complex = ditfft2(&x_complex[..]);

    for (i, n) in ret_complex.iter().enumerate() {
        X[i] = n.re;
    }
}

// Cooley-Tukey FFT algorithm from wikipedia
//
// X0,...,N−1 ← ditfft2(x, N, s):             DFT of (x0, xs, x2s, ..., x(N-1)s):
//     if N = 1 then
//         X0 ← x0                                      trivial size-1 DFT base case
//     else
//         X0,...,N/2−1 ← ditfft2(x, N/2, 2s)             DFT of (x0, x2s, x4s, ...)
//         XN/2,...,N−1 ← ditfft2(x+s, N/2, 2s)           DFT of (xs, xs+2s, xs+4s, ...)
//         for k = 0 to N/2−1                           combine DFTs of two halves into full DFT:
//             t ← Xk
//             Xk ← t + exp(−2πi k/N) Xk+N/2
//             Xk+N/2 ← t − exp(−2πi k/N) Xk+N/2
//         endfor
//     endif

/// Cooley-Tukey complex FFT
fn ditfft2(x: &[Complex<f64>]) -> Vec<Complex<f64>> {
    let N = x.len();
    let mut X: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); N];

    if N == 1 {
        X[0] = x[0];
    } else {
        let evens: Vec<Complex<f64>> = x.iter().enumerate()
            .filter(|&(i, _)| i % 2 == 0)
            .map(|(_, &n)| n)
            .collect();
        let odds: Vec<Complex<f64>> = x.iter().enumerate()
            .filter(|&(i, _)| i % 2 == 1)
            .map(|(_, &n)| n)
            .collect();

        let X_even = ditfft2(&evens[..]);
        let X_odd = ditfft2(&odds[..]);

        let upper_bound = match N {
            2 => 1,
            _ => N / 2 - 1
        };

        for k in 0..upper_bound {
            let factor = cis(-2.0 * PI * (k as f64) / (N as f64));
            X[k] = X_even[k] + factor * X_odd[k];
            X[k + N / 2] = X_even[k] - factor * X_odd[k];
        }
    }

    X
}

/// Euler's formula
fn cis(x: f64) -> Complex<f64> {
    let re = x.cos();
    let im = x.sin();
    Complex::new(re, im)
}
