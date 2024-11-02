pub mod arbitrary;
pub mod eq;
pub mod prelude;

#[derive(Debug, Copy, Clone)]
struct Symbol {
    pub name: char,
    pub subscript: Option<u32>,
}

impl Symbol {
    pub fn new(name: char, subscript: Option<u32>) -> Symbol {
        Symbol { name, subscript }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_fn() {
        assert!(true);
    }
}
