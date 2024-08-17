use position::Position;
use crate::{Block, Expression, ForeignExport, FunctionDefinition, Identifier, Lambda};

#[derive(Debug, PartialEq)]
pub struct FunctionDec {
    name: Identifier,                        // 函数名称
    params: Vec<Parameter>,              // 参数列表
    block: Block,
    position: Position,

}


impl FunctionDec {
    pub fn new(
        name: Identifier,
        params: Vec<Parameter>,              // 参数列表
        block: Block,
        position: Position,
    ) -> Self {
        Self {
            name,
            params,
            block,
            position,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn block(&self) -> &Block {
        &self.block
    }
    pub fn position(&self) -> &Position {
        &self.position
    }
}

// 参数定义
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    name: String,                        // 参数名称
    default_value: Option<Expression>,         // 可选的默认值
    position: Position,
}

impl Parameter {
    pub fn new(
        name: String,
        default_value: Option<Expression>,
        position: Position,
    ) -> Self {
        Self {
            name,
            default_value,
            position,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }


    pub fn position(&self) -> &Position {
        &self.position
    }
}