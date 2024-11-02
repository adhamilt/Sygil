use crate::symbol::Symbol;
use quickcheck::Arbitrary;

impl Arbitrary for Symbol {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        if (bool::arbitrary(g)) {
            Symbol::new(char::arbitrary(g), Some(u32::arbitrary(g)))
        } else {
            Symbol::new(char::arbitrary(g), None)
        }
    }
}
