use std::{
    cell::RefCell,
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::{ast::Ast, diagnostic::DiagnosticReporter, get_file_content, utils::FileId};

const FILE_EXT: &str = "olive";

#[derive(Debug)]
pub struct Compiler {
    pub curr_source_content: String,
    modules_entry_dir: PathBuf,
    modules: Vec<OsString>,
    curr_file_id: FileId,
    pub reporter: RefCell<DiagnosticReporter>,
    dump_ast: bool,
}

impl Compiler {
    pub fn new(filepath: &OsStr, dump_ast: bool) -> Self {
        Self {
            // project: Project::new(),
            curr_source_content: get_file_content(Path::new(filepath)),
            modules_entry_dir: PathBuf::from(filepath).parent().unwrap().to_path_buf(),
            modules: vec![PathBuf::from(filepath).file_stem().unwrap().into()],
            curr_file_id: 0,
            reporter: RefCell::new(DiagnosticReporter::new()),
            dump_ast,
        }
    }

    pub fn add_module(&mut self, name: impl Into<OsString>) {
        self.modules.push(name.into());
        // if self.get_filepath(self.modules.len() - 1).is_file() {
        //     self.reporter.borrow_mut().add(error(crate::diagnostic::CompilerError::InvalidModuleFound, ));
        // }
    }

    pub fn next_file(&mut self) {
        if self.curr_file_id + 1 <= self.modules.len() {
            self.curr_file_id += 1;
        }
    }

    pub fn set_file_content(&mut self) {
        self.curr_source_content = get_file_content(&self.get_filepath(self.curr_file_id))
    }

    pub fn get_curr_file_id(&self) -> FileId {
        self.curr_file_id
    }

    pub fn get_filepath(&self, file_id: FileId) -> PathBuf {
        let mut filepath = self.modules_entry_dir.clone();
        filepath.push(self.get_modulename(file_id));
        filepath.with_extension(FILE_EXT)
    }

    pub fn get_modulename(&self, file_id: FileId) -> &OsStr {
        &self.modules[file_id]
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
