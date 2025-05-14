use std::cell::RefCell;
use std::fs::File;
use std::rc::{Rc, Weak};

pub struct FileSystem{
    root: FSItem::Directory,
    working_directory: FSItem, // sostituisci con Rc
}

struct Node {
    name: String,
    node: FSItem,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

enum FSItem {
    Directory(Directory),
    File(File_),
    SymLin(Link)
}

struct Directory {

}

struct File_ {

}

struct Link {

}

impl FileSystem {
    // crea un nuovo FS vuoto
    pub fn new() -> Self
    // crea un nuovo FS replicando la struttura su disco
    pub fn from_disk() -> Self
    // cambia la directory corrente, path come in tutti gli altri metodi
    // può essere assoluto o relativo;
    // es: “../sibling” vuol dire torna su di uno e scendi in sibling
    pub fn change_dir(&mut self, path: String) -> Result
    // crea la dir in memoria e su disco
    pub fn make_dir(&self, path: String, name: String) -> Result
    // crea un file vuoto in memoria e su disco
    pub fn make_dir(&self, path: String, name: String) -> Result
    // rinonima file / dir in memoria e su disco
    pub fn rename(&self, path: String, new_name: String) -> Result
    // cancella file / dir in memoria e su disco, se è una dir cancella tutto il contenuto
    pub fn delete(&self, path: String) -> Result
    // cerca l’elemento indicato dal path e restituisci un riferimento
    pub find( & self , path: String) -> Result
}
