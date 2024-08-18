use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use nom_locate::LocatedSpan;
use position::Position;
use std::str;

use nom::character::complete::multispace0;
use nom::InputLength;
use ast::For::In;
use ast::NodeId;
use crate::parser::{config, ParserCtx};
use crate::PineResult;


//
//
// #[derive(Debug, Clone)]
// pub struct ParserContext {
//     // files: HashMap<PathBuf, SourceFile>,
//     // diagnostics: Diagnostics,
//     cur_file_path: PathBuf,
//     pub(crate) identities: BTreeMap<NodeId, Span>,
//     operators_list: HashMap<String, u8>,
//     pub(crate) block_indent: usize,
//     pub(crate) first_indent: Option<usize>,
//     next_node_id: NodeId,
//     source:String,
//     path:String,
//     //structs: HashMap<String, Type>,
//     //pub config: Config,
//     allow_newline_dot: Vec<()>,
// }
//
// impl ParserContext {
//     pub fn new(file_path: PathBuf) -> Self {
//         Self {
//             source: "".parse().unwrap(),
//             //files: HashMap::new(),
//             cur_file_path: file_path,
//             identities: BTreeMap::new(),
//             operators_list: HashMap::new(),
//             block_indent: 0,
//             first_indent: None,
//             next_node_id: 0,
//             path:"".to_string(),
//             // structs: HashMap::new(),
//             // diagnostics: Diagnostics::default(),
//             // config,
//             allow_newline_dot: vec![],
//         }
//     }
//     //
//     // #[cfg(test)]
//     // pub fn new_with_operators(
//     //     file_path: PathBuf,
//     //     operators: HashMap<String, u8>,
//     //     config: Config,
//     // ) -> Self {
//     //     Self {
//     //         files: HashMap::new(),
//     //         cur_file_path: file_path,
//     //         identities: BTreeMap::new(),
//     //         operators_list: operators,
//     //         block_indent: 0,
//     //         first_indent: None,
//     //         next_node_id: 0,
//     //         structs: HashMap::new(),
//     //         diagnostics: Diagnostics::default(),
//     //         config,
//     //         allow_newline_dot: vec![],
//     //     }
//     // }
//     //
//     // pub fn new_from(&self, name: &str, config: Config) -> Self {
//     //     Self {
//     //         files: HashMap::new(),
//     //         cur_file_path: self
//     //             .cur_file_path
//     //             .parent()
//     //             .unwrap()
//     //             .join(name.to_owned() + ".rk"),
//     //         identities: BTreeMap::new(),
//     //         operators_list: HashMap::new(),
//     //         block_indent: 0,
//     //         first_indent: None,
//     //         next_node_id: self.next_node_id,
//     //         structs: HashMap::new(),
//     //         diagnostics: Diagnostics::default(), // FIXME
//     //         config,
//     //         allow_newline_dot: vec![],
//     //     }
//     // }
//     //
//     // pub fn new_std(&self, config: Config) -> Self {
//     //     Self {
//     //         files: HashMap::new(),
//     //         cur_file_path: PathBuf::from("/std/src/lib.rk"),
//     //         identities: BTreeMap::new(),
//     //         operators_list: HashMap::new(),
//     //         block_indent: 0,
//     //         first_indent: None,
//     //         next_node_id: self.next_node_id,
//     //         structs: HashMap::new(),
//     //         diagnostics: Diagnostics::default(),
//     //         config,
//     //         allow_newline_dot: vec![],
//     //     }
//     // }
//     //
//     pub fn new_identity(&mut self, span: Span) -> NodeId {
//         let node_id = self.next_node_id;
//
//         self.next_node_id += 1;
//
//         self.identities.insert(node_id, span);
//
//         node_id
//     }
//
//     pub fn current_file_path(&self) -> &PathBuf {
//         &self.cur_file_path
//     }
//
//     pub fn operators(&self) -> &HashMap<String, u8> {
//         &self.operators_list
//     }
//
//     pub fn add_operator(&mut self, op: String, prec: u8) {
//         self.operators_list.insert(op, prec);
//     }
//
//     // pub fn identities(&self) -> BTreeMap<NodeId, Span> {
//     //     self.identities.clone()
//     // }
//
//     pub fn operators_list(&self) -> HashMap<String, u8> {
//         self.operators_list.clone()
//     }
//
//     // pub fn files(&self) -> HashMap<PathBuf, SourceFile> {
//     //     self.files.clone()
//     // }
//
//     // pub fn diagnostics(&self) -> Diagnostics {
//     //      self.diagnostics.clone()
//     //  }
// }
pub type Input<'a> = LocatedSpan<&'a str, ParserCtx>;

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub column: usize,
}

impl Span {
    pub fn new(start: Input, end: Input) -> Span {
        let len = end.location_offset() - start.location_offset();

        // trim span
        let mut fragment = &start.fragment()[0..len];
        let trim_start = fragment.chars().take_while(|c| c.is_whitespace()).count();
        let trim_end = fragment
            .chars()
            .rev()
            .take_while(|c| c.is_whitespace())
            .count();

        fragment = &fragment[trim_start..(len - trim_end)];

        Span {
            start: start.location_line() as usize,
            end: end.location_line() as usize,
            line: start.location_line(),
            column: start.get_column(),
            // offset: start.get_utf8_column() + trim_start,
            // fragment,
            // source: start.extra.source,
        }
    }

    pub fn empty() -> Span {
        Span {
            start: 0,
            end: 0,
            line: 0,
            column: 0,
        }
    }
    pub fn between(&self, to: Span) -> Span {
        Span {
            start: self.start,
            end: to.end,
            line: self.line,
            column: self.column,
        }
    }

}

impl From<Input<'_>> for Span {
    fn from(value: Input) -> Self {
        Span {
            start: value.location_offset(),
            end: value.location_offset() + value.input_len(),
            line: value.location_line(),
            column: value.naive_get_utf8_column(),
        }
    }
}



pub fn input<'a>(source: &'a str) -> Input<'a> {

    let input = Input::new_extra(source, ParserCtx::new(PathBuf::new(), config::Config::default()));
   return input

}


// pub fn new_input(input: &str) -> Input<'_> {
//     let mut path = PathBuf::new();
//     let ctx = ParserContext::new(path);
//     Input::new_extra(input, ctx)
// }


pub fn position(input: Input) -> Position {
    let input_clone=input.clone();
    let path = "".clone(); // 克隆 path 避免所有权转移

    Position::new(
        path,
        input_clone.location_line() as usize,
        input_clone.get_column(),
        str::from_utf8(input_clone.get_line_beginning()).unwrap(),
    )
}


pub fn spaced<'a, F, O>(mut parser: F) -> impl FnMut(Input<'a>) -> PineResult<O>
    where
        F: FnMut(Input<'a>) -> PineResult<O>,
// I: InputTakeAtPosition,
// <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    return move |i: Input<'a>| {
        let (i, _) = multispace0(i)?;
        let (i, res) = parser(i)?;
        let (i, _) = multispace0(i)?;

        return Ok((i, res));
    };
}
pub fn span<'a, F, O>(mut parser: F) -> impl FnMut(Input<'a>) -> PineResult< (Span, O)>
    where
        F: FnMut(Input<'a>) -> PineResult< O>,
{
    return move |i: Input<'a>| {
        let (start_i, _) = nom_locate::position(i)?;
        let (parsed_i, out) = parser(start_i.clone())?;
        let (i, end) = nom_locate::position(parsed_i.clone())?;

        Ok((i, (Span::new(start_i, end), out)))
    };
}

