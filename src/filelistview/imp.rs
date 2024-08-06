use crate::filelistview;
use chrono::offset::LocalResult;
use chrono::{Local, TimeZone};
use glib::object::ObjectExt;
use glib::subclass::object::{ObjectImpl, ObjectImplExt};
use glib::subclass::types::{ObjectSubclass, ObjectSubclassExt};
use gtk4::glib;
use gtk4::prelude::TreeViewExt;
use gtk4::subclass::prelude::TreeViewImpl;
use gtk4::subclass::widget::WidgetImpl;
use human_bytes::human_bytes;

use super::{Columns, TreeModelMviewExt};

#[derive(Debug, Default)]
pub struct FileListViewImp {}

#[glib::object_subclass]
impl ObjectSubclass for FileListViewImp {
    const NAME: &'static str = "FileListView";
    type Type = filelistview::FileListView;
    type ParentType = gtk4::TreeView;
}

impl ObjectImpl for FileListViewImp {
    fn constructed(&self) {
        self.parent_constructed();
        let instance = self.obj();

        // Column for category
        let renderer = gtk4::CellRendererPixbuf::new();
        let column = gtk4::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        // column.set_title("Cat");
        column.add_attribute(&renderer, "icon-name", Columns::Icon as i32);
        column.set_sizing(gtk4::TreeViewColumnSizing::Fixed);
        column.set_fixed_width(30);
        column.set_sort_column_id(Columns::Cat as i32);
        instance.append_column(&column);

        // Column for file/direcory
        let renderer_txt = gtk4::CellRendererText::new();
        // let renderer_icon = gtk4::CellRendererPixbuf::new();
        // renderer_icon.set_padding(6, 0);
        let column = gtk4::TreeViewColumn::new();
        // column.pack_start(&renderer_icon, false);
        column.pack_start(&renderer_txt, true);
        column.set_title("Name");
        // column.add_attribute(&renderer_icon, "icon-name", Columns::Icon as i32);
        column.add_attribute(&renderer_txt, "text", Columns::Name as i32);
        column.set_sizing(gtk4::TreeViewColumnSizing::Fixed);
        column.set_fixed_width(300);
        column.set_sort_column_id(Columns::Name as i32);
        instance.append_column(&column);

        // Column for size
        let renderer = gtk4::CellRendererText::new();
        renderer.set_property("xalign", 1.0_f32);
        let column = gtk4::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Size");
        column.set_alignment(1.0);
        column.add_attribute(&renderer, "text", Columns::Size as i32);
        column.set_sizing(gtk4::TreeViewColumnSizing::Fixed);
        column.set_fixed_width(90);
        column.set_sort_column_id(Columns::Size as i32);
        column.set_cell_data_func(&renderer, |_col, renderer, model, iter| {
            let size = model.size(iter);
            let modified_text = if size > 0 {
                human_bytes(size as f64)
            } else {
                String::default()
            };
            renderer.set_property("text", modified_text);
        });
        instance.append_column(&column);

        // Column for modified date
        let renderer = gtk4::CellRendererText::new();
        let column = gtk4::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Modified");
        column.set_sizing(gtk4::TreeViewColumnSizing::Fixed);
        column.set_fixed_width(140);
        column.set_sort_column_id(Columns::Modified as i32);
        column.set_cell_data_func(&renderer, |_col, renderer, model, iter| {
            let modified = model.modified(iter);
            let modified_text = if modified > 0 {
                if let LocalResult::Single(dt) = Local.timestamp_opt(modified as i64, 0) {
                    dt.format("%d-%m-%Y %H:%M:%S").to_string()
                } else {
                    String::default()
                }
            } else {
                String::default()
            };
            renderer.set_property("text", modified_text);
        });
        instance.append_column(&column);
    }
}

impl WidgetImpl for FileListViewImp {}

impl TreeViewImpl for FileListViewImp {}

impl FileListViewImp {}
