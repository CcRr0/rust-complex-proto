mod complex {
    use std::ops::{Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
    use std::fmt;
    #[derive(Copy, Clone, Default)]
    pub struct Complex {
        pub real: f64,
        pub imag: f64,
    }
    #[allow(dead_code)]
    impl Complex {
        pub const REAL_UNIT: Self = Self { real: 1.0, imag: 0.0 };
        pub const IMAG_UNIT: Self = Self { real: 0.0, imag: 1.0 };
        #[inline(always)]
        pub fn new(real: f64, imag: f64) -> Self {
            Self { real, imag }
        }
        #[inline(always)]
        pub fn with_real(real: f64) -> Self {
            Self::new(real, 0.0)
        }
        #[inline(always)]
        pub fn with_imag(imag: f64) -> Self {
            Self::new(0.0, imag)
        }
        #[inline(always)]
        pub fn abs(self) -> f64 {
            self.real.hypot(self.imag)
        }
        #[inline(always)]
        pub fn arg(self) -> f64 {
            self.imag.atan2(self.real)
        }
        #[inline(always)]
        pub fn norm(self) -> f64 {
            self.real * self.real + self.imag * self.imag
        }
        #[inline(always)]
        pub fn conj(self) -> Self {
            Self::new(self.real, -self.imag)
        }
        #[inline(always)]
        pub fn exp(self) -> Self {
            let exp_real: f64 = self.real.exp();
            Self::new(
                exp_real * self.imag.cos(),
                exp_real * self.imag.sin(),
            )
        }
        #[inline(always)]
        pub fn ln(self) -> Self {
            Self::new(self.abs().ln(), self.arg())
        }
        #[inline(always)]
        pub fn log2(self) -> Self {
            Self::new(self.abs().log2(), self.arg())
        }
        #[inline(always)]
        pub fn log10(self) -> Self {
            Self::new(self.abs().log10(), self.arg())
        }
        #[inline(always)]
        pub fn log(self, base: f64) -> Self {
            Self::new(self.abs().log(base), self.arg())
        }
        #[inline(always)]
        pub fn sqrt(self) -> Self {
            let abs_sqrt: f64 = self.abs().sqrt();
            let arg_half: f64 = self.arg() * 0.5;
            Self::new(
                abs_sqrt * arg_half.cos(),
                abs_sqrt * arg_half.sin(),
            )
        }
        #[inline(always)]
        pub fn powi(self, exp: i32) -> Self {
            let abs_pow: f64 = self.abs().powi(exp);
            let arg: f64 = self.arg();
            Self::new(
                abs_pow * (exp as f64 * arg).cos(),
                abs_pow * (exp as f64 * arg).sin(),
            )
        }
        #[inline(always)]
        pub fn powf(self, exp: f64) -> Self {
            let abs_pow: f64 = self.abs().powf(exp);
            let arg: f64 = self.arg();
            Self::new(
                abs_pow * (exp * arg).cos(),
                abs_pow * (exp * arg).sin(),
            )
        }
        #[inline(always)] // noinspection SpellCheckingInspection
        pub fn powc(self, exp: Self) -> Self {
            let ln: Self = self.ln();
            Self::new(
                (exp.real * ln.real - exp.imag * ln.imag).exp(),
                exp.real * ln.imag + exp.imag * ln.real,
            )
        }
        #[inline(always)]
        pub fn sin(self) -> Self {
            Self::new(
                self.real.sin() * self.imag.cosh(),
                self.real.cos() * self.imag.sinh(),
            )
        }
        #[inline(always)]
        pub fn cos(self) -> Self {
            Self::new(
                self.real.cos() * self.imag.cosh(),
                -(self.real.sin() * self.imag.sinh()),
            )
        }
        #[inline(always)]
        pub fn tan(&self) -> Self {
            self.sin() / self.cos()
        }
        #[inline(always)]
        pub fn sinh(self) -> Self {
            Self::new(
                self.real.sinh() * self.imag.cos(),
                self.real.cosh() * self.imag.sin(),
            )
        }
        #[inline(always)]
        pub fn cosh(self) -> Self {
            Self::new(
                self.real.cosh() * self.imag.cos(),
                self.real.sinh() * self.imag.sin(),
            )
        }
        #[inline(always)]
        pub fn tanh(self) -> Self {
            self.sinh() / self.cosh()
        }
        #[inline(always)]
        pub fn asin(self) -> Self {
            -Self::IMAG_UNIT * (Self::IMAG_UNIT * self + (-(self * self) + 1.0_f64).sqrt()).ln()
        }
        #[inline(always)]
        pub fn acos(self) -> Self {
            -Self::IMAG_UNIT * (self + Self::IMAG_UNIT * (-(self * self) + 1.0_f64).sqrt()).ln()
        }
        #[inline(always)]
        pub fn atan(self) -> Self {
            Self::IMAG_UNIT * Self::with_real(0.5) * (
                (-(Self::IMAG_UNIT * self) + 1.0_f64) / (Self::IMAG_UNIT * self + 1.0_f64)
            ).ln()
        }
        #[inline(always)]
        pub fn asinh(self) -> Self {
            (self + (self * self + 1.0_f64).sqrt()).ln()
        }
        #[inline(always)]
        pub fn acosh(self) -> Self {
            (self + (self * self - 1.0_f64).sqrt()).ln()
        }
        #[inline(always)]
        pub fn atanh(self) -> Self {
            Self::with_real(0.5) * ((self + 1.0_f64) / (-self + 1.0_f64)).ln()
        }
    }
    impl Neg for Complex {
        type Output = Self;
        #[inline(always)]
        fn neg(self) -> Self::Output {
            Self::new(
                -self.real,
                -self.imag,
            )
        }
    }
    impl Add for Complex {
        type Output = Self;
        #[inline(always)]
        fn add(self, other: Self) -> Self::Output {
            Self::new(
                self.real + other.real,
                self.imag + other.imag,
            )
        }
    }
    impl AddAssign for Complex {
        #[inline(always)]
        fn add_assign(&mut self, other: Self) -> () {
            self.real += other.real;
            self.imag += other.imag;
        }
    }
    impl Add<f64> for Complex {
        type Output = Self;
        #[inline(always)]
        fn add(self, rhs: f64) -> Self::Output {
            Self::new(
                self.real + rhs,
                self.imag,
            )
        }
    }
    impl AddAssign<f64> for Complex {
        #[inline(always)]
        fn add_assign(&mut self, rhs: f64) -> () {
            self.real += rhs;
        }
    }
    impl Sub for Complex {
        type Output = Self;
        #[inline(always)]
        fn sub(self, other: Self) -> Self::Output {
            Self::new(
                self.real - other.real,
                self.imag - other.imag,
            )
        }
    }
    impl SubAssign for Complex {
        #[inline(always)]
        fn sub_assign(&mut self, other: Self) -> () {
            self.real -= other.real;
            self.imag -= other.imag;
        }
    }
    impl Sub<f64> for Complex {
        type Output = Self;
        #[inline(always)]
        fn sub(self, rhs: f64) -> Self::Output {
            Self::new(
                self.real - rhs,
                self.imag,
            )
        }
    }
    impl SubAssign<f64> for Complex {
        #[inline(always)]
        fn sub_assign(&mut self, rhs: f64) -> () {
            self.real -= rhs;
        }
    }
    impl Mul for Complex {
        type Output = Self;
        #[inline(always)]
        fn mul(self, other: Self) -> Self::Output {
            Self::new(
                self.real * other.real - self.imag * other.imag,
                self.real * other.imag + self.imag * other.real,
            )
        }
    }
    impl MulAssign for Complex {
        #[inline(always)]
        fn mul_assign(&mut self, other: Self) -> () {
            (self.real, self.imag) = (
                self.real * other.real - self.imag * other.imag,
                self.real * other.imag + self.imag * other.real,
            );
        }
    }
    impl Mul<f64> for Complex {
        type Output = Self;
        #[inline(always)]
        fn mul(self, rhs: f64) -> Self::Output {
            Self::new(
                self.real * rhs,
                self.imag * rhs,
            )
        }
    }
    impl MulAssign<f64> for Complex {
        #[inline(always)]
        fn mul_assign(&mut self, rhs: f64) -> () {
            self.real *= rhs;
            self.imag *= rhs;
        }
    }
    impl Div for Complex {
        type Output = Self;
        #[inline(always)]
        fn div(self, other: Self) -> Self::Output {
            let denom: f64 = other.norm();
            Self::new(
                (self.real * other.real + self.imag * other.imag) / denom,
                (self.imag * other.real - self.real * other.imag) / denom,
            )
        }
    }
    impl DivAssign for Complex {
        #[inline(always)]
        fn div_assign(&mut self, other: Self) -> () {
            let denom: f64 = other.norm();
            (self.real, self.imag) = (
                (self.real * other.real + self.imag * other.imag) / denom,
                (self.imag * other.real - self.real * other.imag) / denom,
            );
        }
    }
    impl Div<f64> for Complex {
        type Output = Self;
        #[inline(always)]
        fn div(self, rhs: f64) -> Self::Output {
            Self::new(
                self.real / rhs,
                self.imag / rhs,
            )
        }
    }
    impl DivAssign<f64> for Complex {
        #[inline(always)]
        fn div_assign(&mut self, rhs: f64) -> () {
            self.real /= rhs;
            self.imag /= rhs;
        }
    }
    impl fmt::Display for Complex {
        #[inline(always)]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if let Some(precision) = f.precision() {
                write!(f, "{:.precision$}{:+.precision$}i", self.real, self.imag, precision = precision)
            } else {
                write!(f, "{}{:+}i", self.real, self.imag)
            }
        }
    }
    impl fmt::Debug for Complex {
        #[inline(always)]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}{:+}i", self.real, self.imag)
        }
    }
}
use complex::Complex;
