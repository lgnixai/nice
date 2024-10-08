use std::{collections::HashMap, path::PathBuf};

use nom::error::VerboseError;
use log::{trace, warn};
use crate::input::Input;

use crate::parser::{SourceFile, Spaned};
use crate::parser::diagnostic::Diagnostic;



#[derive(Debug, Clone)]
pub enum DiagnosticType {
    Warning,
    Error,
}

impl Default for DiagnosticType {
    fn default() -> Self {
        DiagnosticType::Error
    }
}

#[derive(Debug, Default, Clone)]
pub struct Diagnostics {
    pub list: Vec<Diagnostic>,
    pub list_types: Vec<DiagnosticType>,
    pub must_stop: bool,
}

impl Diagnostics {
    pub fn push_error(&mut self, diag: Diagnostic) {
        self.must_stop = true;

        trace!("Push error diagnostic: {:#?}", diag);

        self.list.push(diag);
        self.list_types.push(DiagnosticType::Error);
    }

    pub fn push_warning(&mut self, diag: Diagnostic) {
        trace!("Push warning: {:#?}", diag);

        self.list.push(diag);
        self.list_types.push(DiagnosticType::Warning);
    }

    pub fn print(&self, files: &HashMap<PathBuf, SourceFile>) {


        print!("来吧来吧");
        // let mut span1 = span.clone();
        // if span1.start == span1.end {
        //     span1.end += 1;
        // }
        //
        // let mut span=Spaned::new(PathBuf::from(filename),span1.start,span1.end);
        //
        // for (i, diag) in self.list.iter().enumerate() {
        //
        //
        //     let input = match files.get(&diag.span.file_path) {
        //         Some(input) => input,
        //         None => {
        //             println!("DIAG FILE {:#?}", diag.span.file_path);
        //             warn!("Diagnostic has been silenced because the file is not found");
        //
        //             continue;
        //         }
        //     };
        //
        //     diag.print(input, self.list_types.get(i).unwrap());
        // }
    }

    pub fn append(&mut self, other: Self) {
        self.list.extend(other.list);
        self.list_types.extend(other.list_types);
        self.must_stop = self.must_stop || other.must_stop;
    }
}
//
// impl<'a> dyn From<VerboseError<Input'a>>> for Diagnostics {
//     fn from(err: VerboseError<Input<'a>>) -> Self {
//         let mut diags = err
//             .errors
//             .clone()
//             .into_iter()
//             .take(1)
//             .map(Diagnostic::from)
//             .collect::<Vec<_>>();
//
//         let diags2 = err
//             .errors
//             .into_iter()
//             .take(1)
//             .map(|(input, _kind)| input.extra.diagnostics().list)
//             .flatten()
//             .collect::<Vec<_>>();
//
//         diags.extend(diags2);
//
//         let mut list = Diagnostics::default();
//
//         for diag in diags {
//             list.push_error(diag);
//         }
//
//         list
//     }
// }
