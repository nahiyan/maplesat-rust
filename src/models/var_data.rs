use super::clause::Clause;

pub struct VarData<'a> {
    pub reason: &'a Clause,
    pub level: i32,
}
