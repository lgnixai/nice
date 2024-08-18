use std::collections::HashMap;
use position::Position;
use crate::{Block, Comment, Expression, ForeignExport, FunctionDefinition, Identifier, Import, Lambda, NodeId, Statement, VariableDefinition};
use crate::ast::utils::{ResolutionMap, TraitSolver};


#[derive(Debug, Clone)]
pub struct Mod {
    pub top_levels: Vec<TopLevel>,
}

impl Mod {
    pub fn new(top_levels: Vec<TopLevel>) -> Self {
        Self { top_levels }
    }
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Variable(VariableDefinition),
    Function(FunctionDecl),
    Comment(Comment),
    Import(Import),
    If(IfDecl),
    While(While),
    For(For),
    Mod(Identifier, Mod),

}

impl TopLevel {
    pub fn new_function(f: FunctionDecl) -> Self {
        Self::Function(f)
    }
    pub fn new_if(f: IfDecl) -> Self {
        Self::If(f)
    }
    pub fn new_while(f: While) -> Self {
        Self::While(f)
    }

    pub fn new_for(f: For) -> Self {
        Self::For(f)
    }
    pub fn new_var(f: VariableDefinition) -> Self {
        Self::Variable(f)
    }

    pub fn new_mod(ident: Identifier, mod_: Mod) -> Self {
        Self::Mod(ident, mod_)
    }
}


#[derive(Debug, Clone)]
pub struct Body {
    pub stmts: Vec<Statement>,
}

impl Body {
    pub fn new(stmts: Vec<Statement>) -> Self {
        Self { stmts }
    }
}


#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: Identifier,
    pub arguments: Vec<Parameter>,
    pub body: Body,
    pub node_id: NodeId,

    pub position: Position,

}

impl FunctionDecl {
    pub fn new_self(
        node_id: NodeId,
        self_node_id: NodeId,
        name: Identifier,
        body: Body,
        mut arguments: Vec<Parameter>,
        position: Position,
    ) -> Self {
        let root = Identifier::new("self".to_string(), self_node_id);
        let rootone = Parameter::new(root, None, position.clone());
        arguments.insert(0, rootone);
        Self {
            name,

            arguments,
            body,
            node_id,
            position,
        }
    }

    pub fn mangle(&mut self, prefixes: &[String]) {
        if prefixes.is_empty() {
            return;
        }

        self.name.name = prefixes.join("_") + "_" + &self.name.name;
    }
}

// 参数定义
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    name: Identifier,                        // 参数名称
    default_value: Option<Expression>,         // 可选的默认值
    position: Position,
}

impl Parameter {
    pub fn new(
        name: Identifier,
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


#[derive(Debug, Clone)]
pub struct IfDecl {
    pub node_id: NodeId,
    pub predicat: Expression,
    pub body: Body,
    pub else_: Option<Box<Else>>,
}

impl IfDecl {
    pub fn new(
        node_id: NodeId,
        predicat: Expression,
        body: Body,
        else_: Option<Box<Else>>,
    ) -> Self {
        Self {
            node_id,
            predicat,
            body,
            else_,
        }
    }

    pub fn get_flat(&self) -> Vec<(NodeId, Expression, Body)> {
        let mut res = vec![];

        res.push((self.node_id, self.predicat.clone(), self.body.clone()));

        if let Some(else_) = &self.else_ {
            res.extend(else_.get_flat());
        }

        res
    }

    pub fn last_else(&self) -> Option<&Body> {
        if let Some(else_) = &self.else_ {
            else_.last_else()
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum Else {
    If(IfDecl),
    Body(Body),
}

impl Else {
    pub fn get_flat(&self) -> Vec<(NodeId, Expression, Body)> {
        match self {
            Else::If(if_) => if_.get_flat(),
            Else::Body(_body) => vec![],
        }
    }

    pub fn last_else(&self) -> Option<&Body> {
        match self {
            Else::If(if_) => if_.last_else(),
            Else::Body(body) => Some(body),
        }
    }
}


#[derive(Debug, Clone)]
pub enum For {
    In(ForIn),
    While(While),
}

#[derive(Debug, Clone)]
pub struct While {
    pub predicat: Expression,
    pub body: Body,
}

impl While {
    pub fn new(predicat: Expression, body: Body) -> Self {
        Self { predicat, body }
    }
}

#[derive(Debug, Clone)]
pub struct ForIn {
    pub value: Identifier,
    pub expr: Expression,
    pub body: Body,
}

impl ForIn {
    pub fn new(value: Identifier, expr: Expression, body: Body) -> Self {
        Self { value, expr, body }
    }
}