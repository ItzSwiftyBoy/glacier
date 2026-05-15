#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Int(usize),
    IntSized,
    UInt(usize),
    USized,
    Float(usize),
    Void,
    Unknown,
}
