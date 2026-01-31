use std::{
    cell::RefCell,
    ffi::OsStr,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use crate::{ast::Ast, diagnostic::DiagnosticReporter, utils::FileId};

#[derive(Debug)]
pub struct Compiler {
    pub curr_source: String,
    modules: Vec<PathBuf>,
    curr_file_id: FileId,
    pub reporter: RefCell<DiagnosticReporter>,
    dump_ast: bool,
}

impl Compiler {
    pub fn new(filepath: &str, dump_ast: bool) -> Self {
        Self {
            curr_source: Self::get_file_source(Path::new(filepath)),
            modules: vec![PathBuf::from(filepath)],
            curr_file_id: 0,
            reporter: RefCell::new(DiagnosticReporter::new()),
            dump_ast,
        }
    }

    pub fn add_module(&mut self, filename: &str) {
        self.modules.push(PathBuf::from(filename));
    }

    pub fn next_file(&mut self) {
        if self.curr_file_id + 1 < self.modules.len() {
            self.curr_file_id += 1;
        }
    }

    pub fn set_file_source(&mut self) {
        self.curr_source = Self::get_file_source(self.get_module_filepath(self.curr_file_id))
    }

    pub fn get_file_source(filepath: &Path) -> String {
        let mut file = match File::open(filepath) {
            Ok(content) => content,
            Err(r) => {
                eprintln!("Couldn't open the file. Reason: {}", r);
                return String::new();
            }
        };
        let mut source = String::new();
        if file.read_to_string(&mut source).is_err() {
            eprintln!("Got an invalid UTF-8 character!");
            return String::new();
        };

        source
    }

    pub fn get_curr_file_id(&self) -> FileId {
        self.curr_file_id
    }

    pub fn get_module_filepath(&self, file_id: FileId) -> &Path {
        self.modules.get(file_id).unwrap().as_path()
    }

    pub fn get_module_filename(&self, file_id: FileId) -> &OsStr {
        self.modules
            .get(file_id)
            .unwrap()
            .as_path()
            .file_name()
            .unwrap()
    }

    pub fn dump_ast(&self, ast: Ast) {
        if self.dump_ast {
            println!("{:#?}", ast)
        }
    }

    pub fn print_error(&self) {
        if self.reporter.borrow().has_error() {
            self.reporter.borrow().report(self);
        }
    }
}
