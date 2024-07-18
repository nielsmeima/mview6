use crate::{
    backends::TreeModelMviewExt, category::Category, config::config, draw::draw,
    window::MViewWidgets,
};
use eog::Image;
use gtk::{prelude::GtkListStoreExtManual, ListStore, TreeIter};
use std::{cell::RefCell, fs, io, time::UNIX_EPOCH};

use super::{empty_store, Backend, Columns};

pub struct Bookmarks {
    store: ListStore,
    parent: RefCell<Box<dyn Backend>>,
}

impl Default for Bookmarks {
    fn default() -> Self {
        Self::new()
    }
}

impl Bookmarks {
    pub fn new() -> Self {
        Bookmarks {
            store: Self::create_store(),
            parent: RefCell::new(<dyn Backend>::invalid()),
        }
    }

    fn read_directory(store: &ListStore) -> io::Result<()> {
        let config = config();
        for entry in &config.bookmarks {
            let metadata = match fs::metadata(&entry.folder) {
                Ok(m) => m,
                Err(e) => {
                    println!("{}: Err = {:?}", &entry.folder, e);
                    continue;
                }
            };
            let modified = metadata.modified().unwrap_or(UNIX_EPOCH);
            let modified = modified.duration_since(UNIX_EPOCH).unwrap().as_secs();
            let file_size = metadata.len();
            let cat = Category::Direcory;
            store.insert_with_values(
                None,
                &[
                    (Columns::Cat as u32, &cat.id()),
                    (Columns::Icon as u32, &cat.icon()),
                    (Columns::Name as u32, &entry.name),
                    (Columns::Folder as u32, &entry.folder),
                    (Columns::Size as u32, &file_size),
                    (Columns::Modified as u32, &modified),
                ],
            );
        }
        Ok(())
    }

    fn create_store() -> ListStore {
        let store = empty_store();
        match Self::read_directory(&store) {
            Ok(()) => (),
            Err(e) => {
                println!("read_dir failed {:?}", e);
            }
        }
        store
    }
}

impl Backend for Bookmarks {
    fn class_name(&self) -> &str {
        "Bookmarks"
    }

    fn path(&self) -> &str {
        "/bookmarks"
    }

    fn store(&self) -> ListStore {
        self.store.clone()
    }

    fn enter(&self, model: ListStore, iter: TreeIter) -> Box<dyn Backend> {
        <dyn Backend>::new(&model.folder(&iter))
    }

    fn leave(&self) -> (Box<dyn Backend>, Option<String>) {
        (self.parent.borrow().backend().dynbox(), None)
    }

    fn image(&self, _w: &MViewWidgets, model: &ListStore, iter: &TreeIter) -> Image {
        draw(&model.folder(iter)).unwrap()
    }

    fn set_parent(&self, parent: Box<dyn Backend>) {
        self.parent.replace(parent);
    }
}
