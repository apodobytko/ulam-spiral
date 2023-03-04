extern crate gdk;
extern crate gio;
extern crate gtk;

use self::gtk::prelude::*;

use gtk::{
    traits::FileChooserExt, ButtonsType, DialogFlags, FileChooserAction, FileChooserDialog,
    MessageDialog, MessageType, ResponseType, Window, WindowType,
};
use std::path::PathBuf;

pub struct SaveDialog(FileChooserDialog);

pub struct ErrorDialog(MessageDialog);

impl SaveDialog {
    pub fn new() -> SaveDialog {
        let save_dialog = FileChooserDialog::with_buttons(
            Some("Save"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
            &[("Cancel", ResponseType::Cancel), ("Save", ResponseType::Ok)],
        );
        save_dialog.set_current_name("image.png");
        SaveDialog(save_dialog)
    }

    pub fn get_user_choice(&self) -> Option<PathBuf> {
        match self.0.run() {
            ResponseType::Ok => self.0.filename(),
            ResponseType::Cancel => None,

            _ => None,
        }
    }
}

impl Drop for SaveDialog {
    fn drop(&mut self) {
        unsafe {
            self.0.destroy();
        }
    }
}

impl ErrorDialog {
    pub fn show(message: &str) {
        let dialog = MessageDialog::new(
            None::<&Window>,
            DialogFlags::empty(),
            MessageType::Error,
            ButtonsType::Close,
            message,
        );
        dialog.run();
        unsafe {
            dialog.destroy();
        }
    }
}
