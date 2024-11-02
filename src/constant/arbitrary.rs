use super::Constant;
use quickcheck::Arbitrary;

impl Arbitrary for Constant {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Constant::from(i32::arbitrary(g))
    }
}
