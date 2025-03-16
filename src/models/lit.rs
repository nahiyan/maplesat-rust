use super::var::Var;
use std::ops::Not;

pub struct Lit {
    value: i32,
}

// Note: Sign is in the LSB and the var in the MSB
impl Lit {
    pub fn new(var: Var, sign: bool) -> Lit {
        assert!(var >= 0 && var <= 1);
        let value = (var << 1) | (sign as i32);
        Lit { value }
    }

    pub fn sign(&self) -> bool {
        self.value & 1 == 1
    }

    pub fn var(&self) -> Var {
        self.value >> 1
    }

    // Implement operator ^
    pub fn xor(self, b: bool) -> Self {
        Lit {
            value: self.value ^ (b as i32),
        }
    }
}

impl Default for Lit {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl PartialEq for Lit {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Lit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

// Invert the sign
impl Not for Lit {
    type Output = Self;

    fn not(self) -> Self::Output {
        Lit {
            value: self.value ^ 1,
        }
    }
}

// TODO: Implement operator ^

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lit_new() {
        let var = 1;
        let sign = true;
        let lit = Lit::new(var, sign);
        assert_eq!(lit.value, 3);
    }

    #[test]
    fn test_lit_sign() {
        let lit = Lit { value: 3 };
        assert!(lit.sign());

        let lit = Lit { value: 2 };
        assert!(!lit.sign());
    }

    #[test]
    fn test_lit_var() {
        let lit = Lit { value: 3 };
        assert_eq!(lit.var(), 1);

        let lit = Lit { value: 2 };
        assert_eq!(lit.var(), 1);
    }

    #[test]
    fn test_lit_default() {
        let lit = Lit::default();
        assert_eq!(lit.value, 0);
    }

    #[test]
    fn test_lit_partial_eq() {
        let lit1 = Lit { value: 2 };
        let lit2 = Lit { value: 3 };
        let lit3 = Lit { value: 3 };
        assert!(lit1 != lit2);
        assert!(lit2 == lit3);
    }

    #[test]
    fn test_lit_partial_ord() {
        let lit1 = Lit { value: 2 };
        let lit2 = Lit { value: 3 };
        assert!(lit1 < lit2);
        assert!(lit2 > lit1);
    }

    #[test]
    fn test_lit_not() {
        let lit = Lit::new(1, false);
        let lit2 = !lit;
        assert_eq!(lit2.value, 3);
    }
}
