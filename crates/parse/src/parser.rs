pub(crate) mod config;
mod diagnostic;
mod parsing_context;
mod scopes;
pub(crate) mod source_file;
mod diagnostics_list;

pub use config::*;
pub use source_file::*;
pub use diagnostic::*;
pub use diagnostics_list::*;
pub use parsing_context::*;

use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::path::PathBuf;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1};
use nom::combinator::{eof, map};
use nom::error::{ErrorKind, FromExternalError, VerboseError};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated};
use nom_locate::LocatedSpan;
use ast::{Identifier, Mod, NodeId, ResolutionMap, TopLevel, TraitSolver};
use crate::input::{Input, Span};
use crate::parser::config::Config;
use crate::parser::diagnostics_list::Diagnostics;
use crate::parser::source_file::SourceFile;
use crate::{parse_for, parse_function, parse_identifier, parse_if, parse_variable, parse_while, PineResult};
use crate::ty::Type;

#[derive(Debug, Clone)]
pub struct ParserCtx {
    files: HashMap<PathBuf, SourceFile>,
    diagnostics: Diagnostics,
    cur_file_path: PathBuf,
    identities: BTreeMap<NodeId, Span>,
    operators_list: HashMap<String, u8>,
    pub(crate) block_indent: usize,
    pub(crate) first_indent: Option<usize>,
    next_node_id: NodeId,
    structs: HashMap<String, Type>,
    pub config: Config,
    allow_newline_dot: Vec<()>,
}

impl ParserCtx {
    pub fn new(file_path: PathBuf, config: Config) -> Self {
        Self {
            files: HashMap::new(),
            cur_file_path: file_path,
            identities: BTreeMap::new(),
            operators_list: HashMap::new(),
            block_indent: 0,
            first_indent: None,
            next_node_id: 0,
            structs: HashMap::new(),
            diagnostics: Diagnostics::default(),
            config,
            allow_newline_dot: vec![],
        }
    }

    #[cfg(test)]
    pub fn new_with_operators(
        file_path: PathBuf,
        operators: HashMap<String, u8>,
        config: Config,
    ) -> Self {
        Self {
            files: HashMap::new(),
            cur_file_path: file_path,
            identities: BTreeMap::new(),
            operators_list: operators,
            block_indent: 0,
            first_indent: None,
            next_node_id: 0,
            structs: HashMap::new(),
            diagnostics: Diagnostics::default(),
            config,
            allow_newline_dot: vec![],
        }
    }

    pub fn new_from(&self, name: &str, config: Config) -> Self {
        Self {
            files: HashMap::new(),
            cur_file_path: self
                .cur_file_path
                .parent()
                .unwrap()
                .join(name.to_owned() + ".rk"),
            identities: BTreeMap::new(),
            operators_list: HashMap::new(),
            block_indent: 0,
            first_indent: None,
            next_node_id: self.next_node_id,
            structs: HashMap::new(),
            diagnostics: Diagnostics::default(), // FIXME
            config,
            allow_newline_dot: vec![],
        }
    }

    pub fn new_std(&self, config: Config) -> Self {
        Self {
            files: HashMap::new(),
            cur_file_path: PathBuf::from("/std/src/lib.rk"),
            identities: BTreeMap::new(),
            operators_list: HashMap::new(),
            block_indent: 0,
            first_indent: None,
            next_node_id: self.next_node_id,
            structs: HashMap::new(),
            diagnostics: Diagnostics::default(),
            config,
            allow_newline_dot: vec![],
        }
    }

    pub fn new_identity(&mut self, span: Span) -> NodeId {
        let node_id = self.next_node_id;

        self.next_node_id += 1;

        self.identities.insert(node_id, span);

        node_id
    }

    pub fn current_file_path(&self) -> &PathBuf {
        &self.cur_file_path
    }

    pub fn operators(&self) -> &HashMap<String, u8> {
        &self.operators_list
    }

    pub fn add_operator(&mut self, op: String, prec: u8) {
        self.operators_list.insert(op, prec);
    }

    pub fn identities(&self) -> BTreeMap<NodeId, Span> {
        self.identities.clone()
    }

    pub fn operators_list(&self) -> HashMap<String, u8> {
        self.operators_list.clone()
    }

    pub fn files(&self) -> HashMap<PathBuf, SourceFile> {
        self.files.clone()
    }

    pub fn diagnostics(&self) -> Diagnostics {
        self.diagnostics.clone()
    }
}


#[derive(Debug, Clone)]
pub struct Root {
    pub r#mod: Mod,
    pub resolutions: ResolutionMap<NodeId>,
    pub trait_solver: TraitSolver,
    pub operators_list: HashMap<String, u8>,
    pub unused: Vec<NodeId>,
    pub spans: HashMap<NodeId, Span>,
}

pub struct AstPrintContext {
    indent: usize,
}

impl AstPrintContext {
    pub fn new() -> Self {
        Self { indent: 0 }
    }

    pub fn increment(&mut self) {
        self.indent += 1;
    }

    pub fn decrement(&mut self) {
        self.indent -= 1;
    }

    pub fn indent(&self) -> usize {
        self.indent
    }



    pub fn print_primitive<T>(&self, t: T)
        where
            T: Debug,
    {
        let indent_str = String::from("  ").repeat(self.indent());

        println!("{}{:?}", indent_str, t);
    }
}
impl Root {
    pub fn new(r#mod: Mod) -> Self {
        Self {
            r#mod,
            resolutions: ResolutionMap::default(),
            operators_list: HashMap::new(),
            unused: vec![],
            spans: HashMap::new(),
            trait_solver: TraitSolver::new(),
        }
    }

    pub fn print(&self) {

        print!("astprintcontext ");
        //AstPrintContext::new().visit_root(self);
    }
}


pub fn parse_root(input: Input) -> PineResult<Root> {
    // TODO: move eof check in parse_mod
    map(terminated(parse_mod, eof), Root::new)(input)
}

pub fn parse_mod(input: Input) -> PineResult< Mod> {
    map(
        terminated(many1(terminated(parse_top_level, many0(line_ending))), eof),
        Mod::new,
    )(input)
}

pub fn parse_top_level(input: Input) -> PineResult< TopLevel> {
    alt((
        // preceded(
        //     terminated(tag("extern"), space1),
        //     map(parse_prototype, TopLevel::new_extern),
        // ),
        // parse_infix,
        map(parse_variable, TopLevel::new_var),
        map(parse_function, TopLevel::new_function),
        map(parse_if, TopLevel::new_if),
        map(parse_for, TopLevel::new_for),
        map(parse_while, TopLevel::new_while),

        map(parse_mod_decl, |(name, mod_)| TopLevel::new_mod(name, mod_)),
    ))(input)
}

pub fn parse_mod_decl(input: Input) ->  PineResult< (Identifier, Mod)> {
    let config = input.extra.config.clone();

    let (mut input, mod_name) = preceded(terminated(tag("mod"), space1), parse_identifier)(input)?;

    let mut new_ctx = if mod_name.name == "std" {
        input.extra.new_std(config.clone())
    } else {
        input.extra.new_from(&mod_name.name, config.clone())
    };

    let file_path = new_ctx.current_file_path().to_str().unwrap().to_string();

    let mut file = SourceFile::from_file(file_path.clone()).unwrap(); // FIXME: ERRORS ARE swallowed HERE

    // if config.std {
    //     if STDLIB_FILES.get(&file_path).is_none() {
    //         file.content = "use root::std::prelude::(*)\n".to_owned() + &file.content;
    //     }
    // }

    new_ctx
        .files
        .insert(new_ctx.current_file_path().clone(), file.clone());

    input
        .extra
        .files
        .insert(new_ctx.current_file_path().clone(), file.clone());

    let new_parser = Input::new_extra(&file.content, new_ctx.clone());

    use nom::Finish;

    let parsed_mod_opt = parse_mod(new_parser).map_err(|e| e.to_owned()).finish();

    let (input2, mod_) = match parsed_mod_opt {
        Ok((input2, mod_)) => (input2, mod_),
        Err(err) => {
            // input
            //     .extra
            //     .diagnostics
            //     .append(Diagnostics::from(err.clone()));
            //
            // input
            //     .extra
            //     .files
            //     .extend(err.errors.get(0).unwrap().0.extra.files.clone());
            //
            return Err(nom::Err::Failure(VerboseError::from_external_error(
                input,
                ErrorKind::Fail,
                err.to_owned(),
            )));
        }
    };

    // hydrate `input` with the new parser's operators
    // TODO: handle duplicate operators
    input
        .extra
        .operators_list
        .extend(input2.extra.operators_list);

    input.extra.diagnostics.append(input2.extra.diagnostics);

    // extend identities
    input.extra.next_node_id = input2.extra.next_node_id;
    input.extra.identities.extend(input2.extra.identities);
    input.extra.files.extend(input2.extra.files);

    Ok((input, (mod_name, mod_)))
}



pub fn parse(parsing_ctx: &mut ParsingCtx) -> Result<Root, Diagnostic> {
    use nom::Finish;

    let content = &parsing_ctx.get_current_file().content;
    println!("content===={:?}",content);
    let mut parser = LocatedSpan::new_extra(
        content.as_str(),
        ParserCtx::new(
            parsing_ctx.get_current_file().file_path.clone(),
            parsing_ctx.config.clone(),
        ),
    );

    parser.extra.files.insert(
        parsing_ctx.get_current_file().file_path.clone(),
        parsing_ctx.get_current_file().clone(),
    );

    let ast = parse_root(parser).finish();

    let ast = match ast {
        Ok((ctx, mut ast)) => {
            //default_impl_populator::populate_default_impl(&mut ast);

            parsing_ctx.identities = ctx.extra.identities();
            parsing_ctx.files.extend(ctx.extra.files());

            ast.operators_list = ctx.extra.operators_list();
            ast.spans = ctx.extra.identities().into_iter().collect();

            // Debug ast
            if parsing_ctx.config.show_ast {
                ast.print();
            }

            Ok(ast)
        }
        Err(e) => {
            parsing_ctx
                .files
                .extend(e.errors.get(0).unwrap().clone().0.extra.files());
            //
            // let diagnostics = Diagnostics::from(e);
            //
            // parsing_ctx.diagnostics.append(diagnostics);

            // parsing_ctx.identities = ast.extra.identities();

            parsing_ctx.return_if_error()?;

            Err(parsing_ctx.diagnostics.list.get(0).unwrap().clone())
        }
    }?;

    parsing_ctx.return_if_error()?;

    Ok(ast)
}
