//! Funcionality related to computing Fourier transforms.

use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::sync::Arc;

use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

use rustfft::FFT;

/// Thing which can perform a complex and real FFT computation.
pub trait FastFourierTransform {
    /// Fast Fourier Transform.
    ///
    /// * `input` - input buffer from which to read time-domain samples; may be modified
    /// * `output` - output buffer into which frequency domain buckets will be written
    fn fft(&self, input: &mut [Complex<f64>], output: &mut [Complex<f64>]);

    /// Real-valued FFT. Essentially just ignores imaginary components of
    /// Cooley-Tukey.
    ///
    /// # Params
    /// * `input` - input buffer from which to read time-domain samples; may be modified
    /// * `output` - output buffer into which frequency domain buckets will be written
    fn real_fft(&self, input: &mut [f64], output: &mut [f64]) {
        let output_complex_refcell = self.get_complex_output_buf();
        let mut output_complex = output_complex_refcell.borrow_mut();
        let input_complex_refcell = self.get_complex_input_buf();
        let mut input_complex = input_complex_refcell.borrow_mut();

        for (idx, elt) in input.iter().enumerate() {
            input_complex[idx].re = *elt;
            input_complex[idx].im = 0f64;
        }

        self.fft(&mut input_complex, &mut output_complex);

        for (i, n) in output_complex.iter().enumerate() {
            output[i] = n.re;
        }
    }

    /// Returns a reference to the input buffer
    fn get_complex_input_buf(&self) -> Rc<RefCell<Vec<Complex<f64>>>>;
    ///
    /// Returns a reference to the output buffer
    fn get_complex_output_buf(&self) -> Rc<RefCell<Vec<Complex<f64>>>>;
}

/// Spectralizer-original generic FFT algorithm. Built to learn how the FFT is implemented, not for
/// real-world efficiency.
pub struct SpectralizerFFT{
    complex_input_buf: Rc<RefCell<Vec<Complex<f64>>>>,
    complex_output_buf: Rc<RefCell<Vec<Complex<f64>>>>,
}

impl SpectralizerFFT {
    /// Constructs a new `SpectralizerFFT` instance.
    pub fn new(size: usize) -> Self {
        Self{
            complex_input_buf: Rc::new(RefCell::new(vec![Zero::zero(); size])),
            complex_output_buf: Rc::new(RefCell::new(vec![Zero::zero(); size])),
        }
    }
}

impl FastFourierTransform for SpectralizerFFT {
    #[allow(non_snake_case)]
    fn fft(&self, input: &mut [Complex<f64>], output: &mut [Complex<f64>]) {
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
        assert!(input.len() == output.len());

        let N = input.len();
        let Nf = N as f64;

        if N == 1 {
            output[0] = input[0];
        } else {
            let mut evens: Vec<Complex<f64>> = input.iter().enumerate()
                .filter(|&(i, _)| i % 2 == 0)
                .map(|(_, &n)| n)
                .collect();
            let mut odds: Vec<Complex<f64>> = input.iter().enumerate()
                .filter(|&(i, _)| i % 2 == 1)
                .map(|(_, &n)| n)
                .collect();

            assert!(evens.len() == odds.len());
            assert!(evens.len() == N / 2);

            let mut evens_output = vec![Zero::zero(); evens.len()];
            let mut odds_output = vec![Zero::zero(); odds.len()];

            self.fft(&mut evens, &mut evens_output);
            self.fft(&mut odds, &mut odds_output);

            for k in 0..N/2 {
                let factor = cis(-2.0 * PI * (k as f64) / Nf);
                output[k] = evens_output[k] + factor * odds_output[k];
                output[k + N / 2] = evens_output[k] - factor * odds_output[k];
            }
        }
    }

    fn get_complex_input_buf(&self) -> Rc<RefCell<Vec<Complex<f64>>>> {
        self.complex_input_buf.clone()
    }
    fn get_complex_output_buf(&self) -> Rc<RefCell<Vec<Complex<f64>>>> {
        self.complex_output_buf.clone()
    }
}

/// Wrapper for RustFFT FFT algorithmm.
pub struct RustFFT {
    fft: Arc<dyn FFT<f64>>,
    complex_input_buf: Rc<RefCell<Vec<Complex<f64>>>>,
    complex_output_buf: Rc<RefCell<Vec<Complex<f64>>>>,
}

impl RustFFT {
    pub fn new(size: usize) -> RustFFT {
        let mut planner = FFTplanner::new(false);
        let fft = planner.plan_fft(size);

        Self {
            fft: fft,
            complex_input_buf: Rc::new(RefCell::new(vec![Zero::zero(); size])),
            complex_output_buf: Rc::new(RefCell::new(vec![Zero::zero(); size])),
        }
    }
}

impl FastFourierTransform for RustFFT {
    fn fft(&self, input: &mut [Complex<f64>], output: &mut [Complex<f64>]) {
        self.fft.process(input, output);
    }

    fn get_complex_input_buf(&self) -> Rc<RefCell<Vec<Complex<f64>>>> {
        self.complex_input_buf.clone()
    }
    fn get_complex_output_buf(&self) -> Rc<RefCell<Vec<Complex<f64>>>> {
        self.complex_output_buf.clone()
    }
}

/// Computes `x` windowed by the Hamming function.
#[allow(non_snake_case)]
pub fn hamming_window(x: &mut [f64]) {
    let alpha = 0.53836;
    let beta = 0.46164;
    let N = x.len();
    let Nf = N as f64 - 1.0;
    let pi2 = 2.0 * PI;

    for i in 0..N {
        let n = x[i];
        x[i] = n * (alpha - beta * ((pi2 * i as f64) / Nf).cos());
    }
}

/// Euler's formula
fn cis(x: f64) -> Complex<f64> {
    let re = x.cos();
    let im = x.sin();
    Complex::new(re, im)
}


#[cfg(test)]
mod test {
    use num::Complex;
    use super::FastFourierTransform;

    #[test]
    /// Ensures correctness of ditfft2() in a typical case.
    fn test_spectralizer_fft() {
        let mut input: Vec<Complex<f64>> = (0..8)
            .map(|i| Complex::new(i as f64, 0.0))
            .collect();
        let mut output: Vec<Complex<f64>> = (0..8)
            .map(|i| Complex::new(i as f64, 0.0))
            .collect();
        // calculated with numpy.fft.fft(); insignificantly adjusted for minor
        // differences in floating point calculations
        let expected = vec![
            Complex::new(28.0,                  0.0),
            Complex::new(-4.0,                  9.65685424949238),
            Complex::new(-4.0,                  4.0),
            Complex::new(-4.0,                  1.6568542494923797),
            Complex::new(-4.0,                  0.0),
            Complex::new(-3.9999999999999996,   -1.6568542494923797),
            Complex::new(-3.9999999999999996,   -4.0),
            Complex::new(-3.9999999999999987,   -9.65685424949238),
        ];
        let _fft = super::SpectralizerFFT::new(input.len());
        _fft.fft(&mut input, &mut output);
        assert_eq!(expected, output);
    }

    #[test]
    /// Ensures correctness of ditfft2() in a typical case.
    fn test_rust_fft() {
        let mut input: Vec<Complex<f64>> = (0..8)
            .map(|i| Complex::new(i as f64, 0f64))
            .collect();
        let mut output: Vec<Complex<f64>> = (0..8)
            .map(|i| Complex::new(i as f64, 0f64))
            .collect();
        // calculated with numpy.fft.fft(); insignificantly adjusted for minor
        // differences in floating point calculations
        let expected = vec![
            Complex::new(28.0,                  0.0),
            Complex::new(-4.0,                  9.65685424949238),
            Complex::new(-4.0,                  4.0),
            Complex::new(-3.9999999999999996,   1.6568542494923797),
            Complex::new(-4.0,                  0.0),
            Complex::new(-3.9999999999999996,   -1.6568542494923797),
            Complex::new(-4.0,                  -4.0),
            Complex::new(-4.0,                  -9.65685424949238),
        ];

        let _fft = super::RustFFT::new(8);
        _fft.fft(&mut input, &mut output);
        assert_eq!(expected, output);
    }

    #[test]
    /// cloning RefCell clones the contained value
    fn refcell_clone_test() {
        let rc = std::cell::RefCell::new(0);

        assert_eq!(rc, rc);
        assert_eq!(*rc.borrow(), *rc.borrow());

        let rc2 = rc.clone();

        assert_eq!(rc, rc2);
        assert_eq!(0, *rc.borrow());
        assert_eq!(0, *rc2.borrow());

        *rc2.borrow_mut() += 1;
        assert_eq!(1, *rc2.borrow());
        assert_eq!(0, *rc.borrow());
    }

    #[test]
    /// cloning Rc<RefCell<_>> does not clone the contained value
    fn rc_refcell_clone_test() {
        let rc = std::rc::Rc::new(std::cell::RefCell::new(0));
        let rc2 = std::rc::Rc::clone(&rc);

        assert_eq!(rc, rc);
        assert_eq!(*rc.borrow(), *rc.borrow());

        assert_eq!(rc, rc2);
        assert_eq!(0, *rc.borrow());
        assert_eq!(0, *rc2.borrow());

        *rc2.borrow_mut() += 1;
        assert_eq!(1, *rc2.borrow());
        assert_eq!(1, *rc.borrow());
    }
}
