use super::{lbool::LBool, var::Var};
use std::ops::Not;

pub struct Lit {
    value: i32,
}

// Note: Sign is in the LSB and the var in the MSB
impl Lit {
    pub fn new(var: Var, sign: bool) -> Lit {
        assert!(var >= 0 && var <= 1);
        let value = (var.id << 1) | (sign as i32);
        Lit { value }
    }

    pub fn sign(&self) -> bool {
        self.value & 1 == 1
    }

    pub fn var(&self) -> Var {
        Var::from(self.value >> 1)
    }

    pub fn value(&self, values: &Vec<LBool>) -> LBool {
        let value = self.var().value(values);
        LBool::from((value as i32) ^ (self.sign() as i32))
    }
}

impl From<i32> for Lit {
    fn from(value: i32) -> Self {
        Lit { value }
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
        let var = Var::from(1);
        let sign = true;
        let lit = Lit::new(var, sign);
        assert_eq!(lit.value, 3);
    }

    #[test]
    fn test_lit_sign() {
        let lit = Lit::from(3);
        assert!(lit.sign());

        let lit = Lit::from(2);
        assert!(!lit.sign());
    }

    #[test]
    fn test_lit_var() {
        let lit = Lit::from(3);
        assert!(lit.var() == 1);

        let lit = Lit::from(2);
        assert!(lit.var() == 1);
    }

    #[test]
    fn test_lit_default() {
        let lit = Lit::default();
        assert_eq!(lit.value, 0);
    }

    #[test]
    fn test_lit_partial_eq() {
        let lit1 = Lit::from(2);
        let lit2 = Lit::from(3);
        let lit3 = Lit::from(3);
        assert!(lit1 != lit2);
        assert!(lit2 == lit3);
    }

    #[test]
    fn test_lit_partial_ord() {
        let lit1 = Lit::from(2);
        let lit2 = Lit::from(3);
        assert!(lit1 < lit2);
        assert!(lit2 > lit1);
    }

    #[test]
    fn test_lit_not() {
        let lit = Lit::new(Var::from(1), false);
        let lit2 = !lit;
        assert_eq!(lit2.value, 3);
    }
}
