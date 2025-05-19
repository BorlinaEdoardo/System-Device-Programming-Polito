/* alzo bandiera bianca...

use std::cell::RefCell;
use std::fs::{File, Metadata};
use std::path::Path;
use std::rc::{Rc, Weak};
use walkdir::WalkDir;

pub struct FileSystem{
    root: Option<Rc<RefCell<Node>>>,
    working_directory: Weak<RefCell<Node>>, // sostituisci con Rc ?
}

struct Node {
    path: String,
    inner: FSItem,
    //children: RefCell<Vec<Rc<Node>>>,
    //parent: RefCell<Weak<Node>>
}

enum FSItem {
    Directory(Directory),
    File(File_),
    SymLin(Link)
}

impl FSItem {

}

struct Directory {
    name: String,
    metadata: Metadata,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

struct File_ {
    name: String,
    metadata: Metadata,
    parent: RefCell<Weak<Node>>
}

struct Link {
    path: Weak<FSItem>,
    parent: RefCell<Weak<Node>>
}

impl FileSystem {
    // crea un nuovo FS vuoto
    pub fn new() -> Self{
        FileSystem{
            root: None,
            working_directory: Weak::new()
        }
    }
    // crea un nuovo FS replicando la struttura su disco

    pub fn from_disk() -> Self{
        let mut filesystem = FileSystem::new();
        let mut node;

        filesystem.root = Some(Rc::new(
            RefCell::new(
                Node{
                    path: "/".to_string(),
                    inner: FSItem::Directory(Directory{
                        name: "root".to_string(),
                        metadata: "root".into(),
                        children: RefCell::new(vec![]),
                        parent: RefCell::new(Default::default()),
                    })
                }
            )
        ));

        filesystem.explore_disk_r(0, 3, "/".to_string());

        filesystem
    }

    fn explore_disk_r(&mut self, depth: i32, max_depth: i32, actual_path: String){
        for entry in WalkDir::new(actual_path)
            .max_depth(1)
            .follow_links(true).into_iter()
            .filter_map(|e| e.ok()) {

            self.explore_disk_r(depth + 1, max_depth, entry.path().to_string());

        }
    }

    /*
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


     */


}

#[test]
pub fn test_filesystem(){
    FileSystem::from_disk();
}

 */

