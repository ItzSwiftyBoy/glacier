#[derive(Debug)]
pub struct Compiler<'a> {
    pub source: &'a str,
}

impl<'a> Compiler<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source }
    }
}
