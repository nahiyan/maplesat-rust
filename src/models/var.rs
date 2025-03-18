use super::lbool::LBool;

#[derive(Clone)]
pub struct Var {
    pub id: i32,
}

impl Var {
    pub fn new(id: i32) -> Self {
        Var { id }
    }

    pub fn into_usize(&self) -> usize {
        self.id as usize
    }

    pub fn value(&self, values: &Vec<LBool>) -> LBool {
        values[self.id as usize].clone()
    }
}

impl PartialOrd<i32> for Var {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other))
    }
}

impl PartialEq<i32> for Var {
    fn eq(&self, other: &i32) -> bool {
        self.id == *other
    }
}
impl From<i32> for Var {
    fn from(id: i32) -> Self {
        Var::new(id)
    }
}
impl From<usize> for Var {
    fn from(id: usize) -> Self {
        Var::new(id as i32)
    }
}

impl Into<usize> for Var {
    fn into(self) -> usize {
        self.id as usize
    }
}
