#[derive(Debug, PartialEq, PartialOrd)]
pub struct Compiler<'a> {
    pub source: &'a [u8],
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self { source }
    }
}
