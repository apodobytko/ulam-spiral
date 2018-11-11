
extern crate gdk;
extern crate gio;
extern crate gtk;

use self::gtk::prelude::*;


use gtk::{FileChooserAction, FileChooserDialog, ResponseType, Window, WindowType};

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
        save_dialog.run();
        SaveDialog(save_dialog)
    }
}

impl Drop for SaveDialog {
    fn drop(&mut self) { self.0.destroy(); }
}