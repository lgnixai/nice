
#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationMode {
    Var,
    Varip,
    Const,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int,
    Float,
    Bool,
    Color,
    String,
    Line,
    LineFill,
    Label,
    Box,
    Table,
    Array(Box<DataType>),
    Matrix(Box<DataType>),
    UDF,
    // 其他类型
}