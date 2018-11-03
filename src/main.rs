extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate image;

mod spiral;

use std::env::args;
use self::gio::prelude::*;
use self::gtk::prelude::*;

use gtk::{WindowPosition};

use self::spiral::Spiral;

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Ulam Spiral Generator");
    window.set_position(WindowPosition::Center);
    window.set_default_size(500, 500);

    let box_vert = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let button = gtk::Button::new_with_label("Generate spiral");
    box_vert.pack_start(&button, false, false, 5);
    window.add(&box_vert);

    let box_weak = box_vert.downgrade();
    let window_weak = window.downgrade();

    button.connect_clicked(move |_| {
        let window = match window_weak.upgrade() {
            Some(window) => window,
            None => return
        };

        let box_vert = match box_weak.upgrade() {
            Some(b) => b,
            None => return
        };

        let spiral = Spiral { x_size: 500, y_size: 500 };
        let image_gtk = spiral.generate_to_gtk();

        box_vert.remove(&image_gtk);
        window.remove(&box_vert);

        box_vert.add(&image_gtk);
        window.add(&box_vert);
        window.show_all();

    });

    window.connect_delete_event(|win, _| {
        win.destroy();
        Inhibit(false)
    });
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        "com.ulam.spiral",
        gio::ApplicationFlags::empty()).expect("Initialization failed.");
    application.connect_startup(|app| {

        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());

}
