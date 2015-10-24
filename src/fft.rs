use num::Complex;
use std::f64::consts::PI;

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


pub fn real_fft(x: &[f64], X: &mut [f64]) {
    assert!(x.len() == X.len());
    let x_complex: Vec<Complex<f64>> = x.iter().map(|&n| Complex::new(n, 0.0)).collect();
    let ret_complex = ditfft2(&x_complex[..], x_complex.len(), 1);

    for (i, n) in ret_complex.iter().enumerate() {
        X[i] = n.re;
    }
}

fn ditfft2(x: &[Complex<f64>], N: usize, s: usize) -> Vec<Complex<f64>> {
    let mut X: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); N];

//  println!("N: {}, x.len: {}, X.len: {}", N, x.len(), X.len());

    if N == 1 {
        X[0] = x[0];
    } else {
        let evens: Vec<Complex<f64>> = x
            .iter()
            .enumerate()
            .filter(|&(i, n)| i % 2 == 0).map(|(_, &n)| n)
            .collect();
        let odds: Vec<Complex<f64>> = x
            .iter()
            .enumerate()
            .filter(|&(i, n)| i % 2 != 0).map(|(_, &n)| n)
            .collect();
        let X_even = ditfft2(&evens[..], N / 2, s * 2);
        let X_odd = ditfft2(&odds[..], N / 2, s * 2);

        // zip up evens and odds
        for i in 0..N {
            X[i] = if i % 2 == 0 {
                X_even[i / 2]
            } else {
                X_odd[(i - 1) / 2]
            };
        }

        for k in 0..N / 2 - 1 {
            let t = X[k];
            X[k] = t + cis(-2.0 * PI * (k as f64) * (N as f64) / 2.0) * X[k + N / 2];
            X[k + N / 2] = t - cis(-2.0 * PI * (k as f64) * (N as f64) / 2.0) * X[k + N / 2];
        }
    }

    X
}

/// Euler's formula
fn cis(ix: f64) -> Complex<f64> {
    let re = ix.cos();
    let im = ix.sin();
    Complex::new(re, im)
}
