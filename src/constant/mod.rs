pub mod add;
pub mod arbitrary;
pub mod eq;
pub mod hash;
pub mod prelude;

#[derive(Clone, Copy, Debug)]
pub struct Constant {
    pub(super) value: i32,
}

impl From<i32> for Constant {
    fn from(value: i32) -> Constant {
        Constant { value }
    }
}

impl From<Constant> for i32 {
    fn from(value: Constant) -> i32 {
        value.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn from_i32(value: i32) {
        let c = Constant::from(value);
        assert_eq!(value, c.value);
        let v2: i32 = c.into();
        assert_eq!(value, v2);
    }
}
