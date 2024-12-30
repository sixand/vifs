use crate::inode::Inode;
use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    rc::Rc,
};

pub trait DentryOp {
    fn dentry_add();
    fn dentry_delete();
    fn dentry_find();
    fn dentry_list();
    fn dentry_parent();
    fn dentry_children();
}


struct DentrysTable {
    tables: Vec<Dentry>,
}

impl DentrysTable {
    fn new() -> Self {
        Self { tables: Vec::new() }
    }
}

#[derive(Clone)]
pub(crate) struct Dentry {
    endpoint: String,
    parent: Option<usize>,
    inode: Box<Inode>,
    tables: Rc<DentrysTable>,
}

impl Dentry {
    pub(crate) fn new(endpoint: String, parent: Option<usize>) -> Self {
        let root = Dentry {
            endpoint,
            parent,
            tables: Rc::new(DentrysTable::new()),
            inode: Box::new(Inode::new(0)),
        };

        root
    }
}

