use crate::symbol::Symbol;

impl PartialEq<Symbol> for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        (self.name == other.name) && (self.subscript == other.subscript)
    }
}

impl Eq for Symbol {}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn test_equality(s: Symbol) {
        assert_eq!(s, s);
    }
}
