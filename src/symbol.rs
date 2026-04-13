use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{
    ast::{Function, Parameter},
    utils::{Span, Token},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScopeKind<'a> {
    Function {
        params: &'a Vec<Parameter>,
        return_ty: &'a Option<Token>,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scope<'a> {
    kind: ScopeKind<'a>,
    span: &'a Span,
    locals: BTreeMap<String, Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn new(kind: ScopeKind<'a>, span: &'a Span) -> Self {
        Self {
            kind,
            span,
            locals: BTreeMap::new(),
        }
    }

    pub fn from_func(func: &'a Function) -> Self {
        Self::new(
            ScopeKind::Function {
                params: &func.params,
                return_ty: &func.return_ty,
            },
            &func.name.span,
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbols<'a> {
    scopes: BTreeMap<String, Scope<'a>>,
}

impl<'a> Symbols<'a> {
    pub fn new() -> Self {
        Self {
            scopes: BTreeMap::new(),
        }
    }

    pub fn add_func(&mut self, func: &'a Function) {
        self.scopes
            .insert(func.name.ty.to_string(), Scope::from_func(func));
    }

    pub fn find_func(&self, name: &str) -> Option<&Scope> {
        self.scopes.get(name)
    }
}

#[derive(Debug)]
pub enum GroupOrModule<'a> {
    Group(HashSet<GroupOrModule<'a>>),
    Module(Symbols<'a>),
}

#[derive(Debug)]
pub struct Project<'a> {
    mod_table: HashMap<String, GroupOrModule<'a>>,
}

impl<'a> Project<'a> {
    pub fn new() -> Self {
        Self {
            mod_table: HashMap::new(),
        }
    }

    pub fn add_module(&mut self, module_name: String) {
        let mut iter = self.mod_table.iter().peekable();
        while let Some(module) = iter.peek_mut() {
            if module.0 == &module_name {
                return;
            }
        }
        self.mod_table
            .insert(module_name, GroupOrModule::Module(Symbols::new()));
    }

    pub fn add_group(&mut self, group_name: String) {
        let mut iter = self.mod_table.iter().peekable();
        while let Some(group) = iter.peek_mut() {
            if group.0 == &group_name {
                return;
            }
        }
        self.mod_table
            .insert(group_name, GroupOrModule::Group(HashSet::new()));
    }
}
