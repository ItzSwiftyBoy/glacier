use std::{
    cell::RefCell,
    ffi::OsStr,
    fs::File,
    io::Read,
    ops::Deref,
    path::{Path, PathBuf},
};

use crate::{ast::Ast, diagnostic::DiagnosticReporter, symbol::Project, utils::FileId};

#[derive(Debug)]
pub struct Compiler {
    pub project: Project<'static>,
    pub curr_source_content: String,
    modules: Vec<PathBuf>,
    curr_file_id: FileId,
    pub reporter: RefCell<DiagnosticReporter>,
    dump_ast: bool,
}

impl Compiler {
    pub fn new(filepath: &str, dump_ast: bool) -> Self {
        Self {
            project: Project::new(),
            curr_source_content: Self::get_file_content(Path::new(filepath)),
            modules: vec![PathBuf::from(filepath)],
            curr_file_id: 0,
            reporter: RefCell::new(DiagnosticReporter::new()),
            dump_ast,
        }
    }

    pub fn add_module(&mut self, filename: &str) {
        self.modules.push(PathBuf::from(filename));

        self.project.add_module(
            self.get_module_filename(self.modules.len() - 1)
                .to_os_string(),
        );
    }

    pub fn next_file(&mut self) {
        if self.curr_file_id + 1 < self.modules.len() {
            self.curr_file_id += 1;
        }
    }

    pub fn set_file_content(&mut self) {
        self.curr_source_content =
            Self::get_file_content(self.get_module_filepath(self.curr_file_id))
    }

    pub fn get_file_content(filepath: &Path) -> String {
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

    pub fn dump_ast(&self, ast: &Ast) {
        if self.dump_ast {
            println!("{:#?}", ast)
        }
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        self.reporter.borrow().report(self);
    }
}
