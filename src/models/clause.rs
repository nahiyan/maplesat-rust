use std::{ops::Index, vec};

use super::lit::Lit;

pub struct Clause {
    // TODO: Consider using a header to group variables together
    mark: u32,
    learnt: u32,
    has_extra: u32,
    reloced: u32,
    size: u32,

    data: Vec<Lit>,
}

pub static UNDEF_CLAUSE: Clause = Clause {
    mark: 0,
    learnt: 0,
    has_extra: 0,
    reloced: 0,
    size: 0,
    data: vec![],
};

impl Index<usize> for Clause {
    type Output = Lit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Clause {
    pub fn iter(&self) -> std::slice::Iter<Lit> {
        self.data.iter()
    }
}

impl<'a> IntoIterator for &'a Clause {
    type Item = &'a Lit;
    type IntoIter = std::slice::Iter<'a, Lit>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}
