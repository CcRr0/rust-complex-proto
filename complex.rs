#[derive(Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}
#[allow(dead_code)]
impl Complex {
    const IMAG_UNIT: Self = Self { real: 0.0, imag: 1.0 };
    const REAL_UNIT: Self = Self { real: 1.0, imag: 0.0 };
    #[inline(always)]
    fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }
    #[inline(always)]
    fn with_real(real: f64) -> Self {
        Self::new(real, 0.0)
    }
    #[inline(always)]
    fn with_imag(imag: f64) -> Self {
        Self::new(0.0, imag)
    }
    #[inline(always)]
    fn abs(self) -> f64 {
        self.real.hypot(self.imag)
    }
    #[inline(always)]
    fn arg(self) -> f64 {
        self.imag.atan2(self.real)
    }
    #[inline(always)]
    fn norm(self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }
    #[inline(always)]
    fn conj(self) -> Self {
        Self::new(self.real, -self.imag)
    }
    #[inline(always)]
    fn exp(self) -> Self {
        let exp_real: f64 = self.real.exp();
        Self::new(
            exp_real * self.imag.cos(),
            exp_real * self.imag.sin(),
        )
    }
    #[inline(always)]
    fn ln(self) -> Self {
        Self::new(self.abs().ln(), self.arg())
    }
    #[inline(always)]
    fn log2(self) -> Self {
        Self::new(self.abs().log2(), self.arg())
    }
    #[inline(always)]
    fn log10(self) -> Self {
        Self::new(self.abs().log10(), self.arg())
    }
    #[inline(always)]
    fn log(self, base: f64) -> Self {
        Self::new(self.abs().log(base), self.arg())
    }
    #[inline(always)]
    fn sqrt(self) -> Self {
        let abs_sqrt: f64 = self.abs().sqrt();
        let arg_half: f64 = self.arg() / 2.0;
        Self::new(
            abs_sqrt * arg_half.cos(),
            abs_sqrt * arg_half.sin(),
        )
    }
    #[inline(always)]
    fn powi(self, exp: i32) -> Self {
        let abs_pow: f64 = self.abs().powi(exp);
        let arg: f64 = self.arg();
        Self::new(
            abs_pow * (exp as f64 * arg).cos(),
            abs_pow * (exp as f64 * arg).sin(),
        )
    }
    #[inline(always)]
    fn powf(self, exp: f64) -> Self {
        let abs_pow: f64 = self.abs().powf(exp);
        let arg: f64 = self.arg();
        Self::new(
            abs_pow * (exp * arg).cos(),
            abs_pow * (exp * arg).sin(),
        )
    }
    #[inline(always)] // noinspection SpellCheckingInspection
    fn powc(self, exp: Self) -> Self {
        let ln: Self = self.ln();
        Self::new(
            (exp.real * ln.real - exp.imag * ln.imag).exp(),
            exp.real * ln.imag + exp.imag * ln.real,
        )
    }
    #[inline(always)]
    fn sin(self) -> Self {
        Self::new(
            self.real.sin() * self.imag.cosh(),
            self.real.cos() * self.imag.sinh(),
        )
    }
    #[inline(always)]
    fn cos(self) -> Self {
        Self::new(
            self.real.cos() * self.imag.cosh(),
            -(self.real.sin() * self.imag.sinh()),
        )
    }
    #[inline(always)]
    fn tan(&self) -> Self {
        self.sin() / self.cos()
    }
    #[inline(always)]
    fn sinh(self) -> Self {
        Self::new(
            self.real.sinh() * self.imag.cos(),
            self.real.cosh() * self.imag.sin(),
        )
    }
    #[inline(always)]
    fn cosh(self) -> Self {
        Self::new(
            self.real.cosh() * self.imag.cos(),
            self.real.sinh() * self.imag.sin(),
        )
    }
    #[inline(always)]
    fn tanh(self) -> Self {
        self.sinh() / self.cosh()
    }
    #[inline(always)]
    fn asin(self) -> Self {
        -Self::IMAG_UNIT * (Self::IMAG_UNIT * self + (Self::REAL_UNIT - self * self).sqrt()).ln()
    }
    #[inline(always)]
    fn acos(self) -> Self {
        -Self::IMAG_UNIT * (self + Self::IMAG_UNIT * (Self::REAL_UNIT - self * self).sqrt()).ln()
    }
    #[inline(always)]
    fn atan(self) -> Self {
        Self::IMAG_UNIT * Self::with_real(0.5) * (
            (Self::REAL_UNIT - Self::IMAG_UNIT * self) / (Self::REAL_UNIT + Self::IMAG_UNIT * self)
        ).ln()
    }
    #[inline(always)]
    fn asinh(self) -> Self {
        (self + (self * self + Self::REAL_UNIT).sqrt()).ln()
    }
    #[inline(always)]
    fn acosh(self) -> Self {
        (self + (self * self - Self::REAL_UNIT).sqrt()).ln()
    }
    #[inline(always)]
    fn atanh(self) -> Self {
        Self::with_real(0.5) * ((Self::REAL_UNIT + self) / (Self::REAL_UNIT - self)).ln()
    }
}
impl std::ops::Neg for Complex {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::new(
            -self.real,
            -self.imag,
        )
    }
}
impl std::ops::Add for Complex {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self::new(
            self.real + other.real,
            self.imag + other.imag,
        )
    }
}
impl std::ops::AddAssign for Complex {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl std::ops::Sub for Complex {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self::new(
            self.real - other.real,
            self.imag - other.imag,
        )
    }
}
impl std::ops::SubAssign for Complex {
    #[inline(always)]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl std::ops::Mul for Complex {
    type Output = Self;
    #[inline(always)]
    fn mul(self, other: Self) -> Self {
        Self::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}
impl std::ops::MulAssign for Complex {
    #[inline(always)]
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
impl std::ops::Div for Complex {
    type Output = Self;
    #[inline(always)]
    fn div(self, other: Self) -> Self {
        let denom: f64 = other.real * other.real + other.imag * other.imag;
        Self::new(
            (self.real * other.real + self.imag * other.imag) / denom,
            (self.imag * other.real - self.real * other.imag) / denom,
        )
    }
}
impl std::ops::DivAssign for Complex {
    #[inline(always)]
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}
