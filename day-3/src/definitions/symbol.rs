#[derive(Debug)]
pub enum SymbolType {
    Usual,
    Gear,
}

#[derive(Debug)]

pub struct Symbol {
    pub index: usize,
    pub variant: SymbolType,
}
