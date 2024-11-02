use crate::constant::Constant;

impl PartialEq for Constant {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_equality(c: Constant) {
        assert_eq!(c, c);
    }
}
