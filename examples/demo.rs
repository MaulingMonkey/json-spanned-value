use codespan_reporting::term::{self, termcolor};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;

use termcolor::{ColorChoice, StandardStream};

use json_spanned_value::{self as jsv, spanned};

fn main() {
    let text = include_str!("demo.json");
    let mut files = SimpleFiles::new();
    let file = files.add("examples/demo.json", text);

    let example : spanned::Object = jsv::from_str(text).unwrap();
    for (k,v) in example {
        emit(&files, &Diagnostic::note().with_message("This is a key!").with_labels(vec![Label::primary(file, k.range())]));
        emit(&files, &Diagnostic::note().with_message("This is a value!").with_labels(vec![Label::primary(file, v.range())]));
    }
}

fn emit(files: &SimpleFiles<&str, &str>, diag: &Diagnostic<usize>) {
    term::emit(
        &mut StandardStream::stdout(ColorChoice::Auto),
        &term::Config::default(),
        files,
        diag
    ).unwrap();
}
