/// Lifted Boolean. In other words, boolean that has undefined value besides
/// true and false.
#[derive(Clone, PartialEq)]
pub enum LBool {
    True = 0,
    False = 1,
    Undefined = 2,
}

impl From<bool> for LBool {
    fn from(value: bool) -> Self {
        match value {
            true => LBool::True,
            false => LBool::False,
        }
    }
}

impl From<i32> for LBool {
    fn from(value: i32) -> Self {
        match value {
            0 => LBool::True,
            1 => LBool::False,
            _ => LBool::Undefined,
        }
    }
}
