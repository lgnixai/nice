use ariadne::{Color, Label, Report, ReportKind, Source};
use std::fmt::Display;


use nom::error::{VerboseError, VerboseErrorKind};
use crate::input::{Input, Span};
use crate::parser::diagnostics_list::DiagnosticType;
use crate::parser::source_file::SourceFile;
use crate::ty::Type;

#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub span: Span,
    kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn new(span: Span, kind: DiagnosticKind) -> Self {
        Self { span, kind }
    }

    pub fn new_empty() -> Self {
        Self {
            span: Span::empty(),
            kind: DiagnosticKind::NoError,
        }
    }

    pub fn new_file_not_found(span: Span, path: String) -> Self {
        Self::new(span, DiagnosticKind::FileNotFound(path))
    }

    pub fn new_unexpected_token(span: Span) -> Self {
        Self::new(span, DiagnosticKind::UnexpectedToken)
    }

    pub fn new_syntax_error(span: Span, msg: String) -> Self {
        Self::new(span, DiagnosticKind::SyntaxError(msg))
    }

    pub fn new_unknown_identifier(span: Span) -> Self {
        Self::new(span, DiagnosticKind::UnknownIdentifier)
    }

    pub fn new_unused_function(span: Span) -> Self {
        Self::new(span, DiagnosticKind::UnusedFunction)
    }

    pub fn new_module_not_found(span: Span, path: String) -> Self {
        Self::new(span, DiagnosticKind::ModuleNotFound(path))
    }

    pub fn new_unresolved_type(span: Span, t: Type) -> Self {
        Self::new(span, DiagnosticKind::UnresolvedType(t))
    }

    pub fn new_out_of_bounds(span: Span, got: u64, expected: u64) -> Self {
        Self::new(span, DiagnosticKind::OutOfBounds(got, expected))
    }



    pub fn new_duplicated_operator(span: Span) -> Self {
        Self::new(span, DiagnosticKind::DuplicatedOperator)
    }

    pub fn new_orphane_signature(span: Span, name: String) -> Self {
        Self::new(span, DiagnosticKind::OrphaneSignature(name))
    }


    pub fn new_no_main() -> Self {
        Self::new(Span::empty(), DiagnosticKind::NoMain)
    }

    pub fn new_is_not_a_property_of(span: Span, span2: Span, t: Type) -> Self {
        Self::new(span, DiagnosticKind::IsNotAPropertyOf(t, span2))
    }

    pub fn new_type_conflict(span: Span, expected: Type, got: Type, in1: Type, in2: Type) -> Self {
        Self::new(span, DiagnosticKind::TypeConflict(expected, got, in1, in2))
    }

    pub fn print(&self, file: &SourceFile, diag_type: &DiagnosticType) {
        self.kind.report_builder(file, &self.span, diag_type);
    }

    pub fn get_kind(&self) -> DiagnosticKind {
        self.kind.clone()
    }
}

#[derive(Clone, Debug)]
pub enum DiagnosticKind {
    FileNotFound(String),
    UnexpectedToken,
    SyntaxError(String),
    UnknownIdentifier,
    ModuleNotFound(String),
    NotAFunction,
    UnusedParameter,

    UnusedFunction,
    DuplicatedOperator,
    TypeConflict(Type, Type, Type, Type), // expected -> got
    UnresolvedType(Type),

    IsNotAPropertyOf(Type, Span),
    OutOfBounds(u64, u64),
    OrphaneSignature(String),

    NoMain,
    NoError, //TODO: remove that
}
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::parser::DiagnosticKind::NoError;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spaned {
    pub file_path: PathBuf,
    pub start: usize,
    pub end: usize,
}

impl Spaned {
    pub fn new(file_path: PathBuf, start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            file_path,
        }
    }

    pub fn new_placeholder() -> Self {
        Self {
            start: 0,
            end: 0,
            file_path: PathBuf::new(),
        }
    }
}
//
// impl<'a> From<Parser<'a>> for Span {
//     fn from(source: Parser<'a>) -> Self {
//         Self {
//             start: source.location_offset(),
//             end: source.to_string().len() + source.location_offset(),
//             file_path: source.extra.current_file_path().clone(),
//         }
//     }
// }

impl DiagnosticKind {
    pub fn report_builder<'a>(
        &self,
        file: &SourceFile,
        span: &'a Span,
        diag_type: &DiagnosticType,
    ) {
        let filename = file.file_path.to_str().unwrap();

        let (error_ty, color) = match diag_type {
            DiagnosticType::Error => (ReportKind::Error, Color::Red),
            DiagnosticType::Warning => (ReportKind::Warning, Color::Yellow),
        };
        let builder = Report::build(error_ty, filename, span.start);

        let mut span1 = span.clone();
        if span1.start == span1.end {
            span1.end += 1;
        }

        let mut span=Spaned::new(PathBuf::from(filename),span1.start,span1.end);



        match self {
            DiagnosticKind::FileNotFound(path) => builder
                .with_message(format!("File not found: {}", path))
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            DiagnosticKind::UnexpectedToken => builder
                .with_message("Unexpected token".to_string())
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            DiagnosticKind::SyntaxError(msg) => builder
                .with_message(format!("Syntax error: {}", msg))
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            DiagnosticKind::UnknownIdentifier => builder
                .with_message("Unknown identifier".to_string())
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            DiagnosticKind::ModuleNotFound(path) => builder
                .with_message(format!("Module not found: {}", path))
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            DiagnosticKind::UnusedFunction => builder
                .with_message("Unused function".to_string())
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            DiagnosticKind::UnusedParameter => builder
                .with_message("Unused parameter".to_string())
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),

            DiagnosticKind::UnresolvedType(t) => builder
                .with_message(format!("Unresolved type: {}", t.to_string()))
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),


            DiagnosticKind::TypeConflict(_t1, _t2, _in1, _in2) => {
                // add spans here
                builder.with_message("Type conflict").with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                )
            }
            DiagnosticKind::OutOfBounds(got, expected) => builder
                .with_message(format!("Out of bounds: got {}, expected {}", got, expected))
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            DiagnosticKind::OrphaneSignature(name) => builder
                .with_message(format!("Orpheline signature: {}", name))
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            // DiagnosticKind::SignatureMismatch(name, _got, _expected) => builder
            //     .with_message(format!("Signature mismatch: {}", name,))
            //     .with_label(
            //         Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
            //             .with_message(format!("{}", self))
            //             .with_color(color),
            //     ),
            DiagnosticKind::NoMain => builder
                .with_message("No main function".to_string())
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            DiagnosticKind::NoError => builder.with_message("No error".to_string()).with_label(
                Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                    .with_message(format!("{}", self))
                    .with_color(color),
            ),
            DiagnosticKind::DuplicatedOperator => builder
                .with_message("Duplicated operator".to_string())
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            DiagnosticKind::NotAFunction => builder
                .with_message("Not a function".to_string())
                .with_label(
                    Label::new((span.file_path.to_str().unwrap(), span.start..span.end))
                        .with_message(format!("{}", self))
                        .with_color(color),
                ),
            _ => {
                todo!()
            }
        }
        .finish()
        .print((filename, Source::from(file.content.clone())))
        .unwrap();
    }
}

impl Display for DiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::UnexpectedToken => "UnexpectedToken".to_string(),
            Self::SyntaxError(msg) => format!("SyntaxError: {}", msg),
            Self::UnknownIdentifier => "UnknownIdentifier".to_string(),
            Self::ModuleNotFound(path) => format!("Module not found: {}", path),
            Self::DuplicatedOperator => "DuplicatedOperator".to_string(),
            Self::TypeConflict(expected, got, _in1, _in2) => {
                use colored::*;
                format!(
                    "Expected {}\n{:<18}But got  {}",
                    format!("{}", expected).blue(),
                    "",
                    format!("{}", got).red(),
                )
            }
            Self::UnresolvedType(t) => {
                format!(
                    "Unresolved type: Type {:?} should be known at this point",
                    t
                )
            }
            // Self::UnresolvedTraitCall {
            //     call_hir_id: _,
            //     given_sig,
            //     existing_impls: _,
            // } => {
            //     format!("Unresolved trait call: {:?}", given_sig,)
            // }
            Self::FileNotFound(path) => format!("FileNotFound {}", path),
            //Self::CodegenError(hir_id, msg) => format!("CodegenError: {} {:?}", msg, hir_id),
            Self::OutOfBounds(got, expected) => format!(
                "Out of bounds error: got indice {} but array len is {}",
                got, expected
            ),
            DiagnosticKind::NotAFunction => "NotAFunction".to_string(),
            DiagnosticKind::UnusedParameter => "UnusedParameter".to_string(),
            DiagnosticKind::UnusedFunction => "UnusedFunction".to_string(),
            DiagnosticKind::OrphaneSignature(_name) => "OrphelineSignature".to_string(),
            // DiagnosticKind::SignatureMismatch(_name, got, expected) => {
            //     use colored::*;
            //     format!(
            //         "Expected {}\n{:<9}But got  {}",
            //         format!("{:?}", expected).blue(),
            //         "",
            //         format!("{:?}", got).red(),
            //     )
            // }
            DiagnosticKind::NoMain => "NoMain".to_string(),
            DiagnosticKind::NoError => "NoError".to_string(),
            DiagnosticKind::IsNotAPropertyOf(t, _span2) => {
                format!("Not a property of {:?}", t)
            }
        };

        write!(f, "{}", s)
    }
}

impl<'a> From<Input<'a>> for Diagnostic {
    fn from(err: Input<'a>) -> Self {
        let span = Span::from(err);

        let msg = "Syntax error".to_string();

        Diagnostic::new_syntax_error(span, msg)
    }
}

impl<'a> From<VerboseError<Input<'a>>> for Diagnostic {
    fn from(err: VerboseError<Input<'a>>) -> Self {
        let (input, _kind) = err.errors.iter().next().unwrap().clone();

        let span = Span::from(input);

        let msg = err.to_string();

        Diagnostic::new_syntax_error(span, msg)
    }
}

impl<I> From<(I, VerboseErrorKind)> for Diagnostic
where
    Span: From<I>,
{
    fn from((input, _kind): (I, VerboseErrorKind)) -> Self {
        let span = Span::from(input);

        let msg = "Syntax error".to_string();

        Diagnostic::new_syntax_error(span, msg)
    }
}
