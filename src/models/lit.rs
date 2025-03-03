use super::var::Var;

pub struct Lit {
    value: i32,
}

impl Lit {
    fn new(var: Var, sign: bool) -> Lit {
        let value = var + var + sign as i32;
        Lit { value }
    }

    fn sign(lit: Lit) -> bool {
        lit.value & 1 == 1
    }

    fn var(lit: Lit) -> Var {
        lit.value >> 1
    }
}

impl PartialEq for Lit {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

// TODO: Check correctness
impl PartialOrd for Lit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

// TODO: Implement operator~ and operator^
