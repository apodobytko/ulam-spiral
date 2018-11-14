
extern crate gdk;
extern crate gio;
extern crate gtk;

use self::gtk::prelude::*;

use std::path::PathBuf;
use gtk::{FileChooserExt, FileChooserAction, FileChooserDialog, ResponseType, Window, WindowType};

pub struct SaveDialog(FileChooserDialog);

impl SaveDialog {
    pub fn new() -> SaveDialog {
        let save_dialog = FileChooserDialog::with_buttons(
            Some("Save"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Save", ResponseType::Ok)]
        );
        SaveDialog(save_dialog)
    }

    pub fn get_user_choice(&self) -> Option<PathBuf> {
        match self.0.run().into() {
            ResponseType::Ok => self.0.get_filename(),
            ResponseType::Cancel => None,
            _ => None,
        }
    }
}

impl Drop for SaveDialog {
    fn drop(&mut self) { self.0.destroy(); }
}