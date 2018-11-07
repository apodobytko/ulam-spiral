extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate image;

mod spiral;

use std::env::args;
use self::gio::prelude::*;
use self::gtk::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use gtk::{WindowPosition};

use self::spiral::Spiral;

// Helper macro that clones variables into a closure.
// Borrowed from Gtk examples repo https://github.com/gtk-rs/examples/.
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn create_main_window(app: &gtk::Application) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Ulam Spiral Generator");
    window.set_position(WindowPosition::Center);
    window.set_default_size(700, 700);
    window
}

fn build_ui(app: &gtk::Application) {
    let window = create_main_window(app);

    let image_map: Rc<RefCell<HashMap<usize, gtk::Image>>> = Rc::new(RefCell::new(HashMap::new()));
    let box_vert = gtk::Box::new(gtk::Orientation::Vertical, 50);
    let button = gtk::Button::new_with_label("Generate spiral");

    let spiral = Spiral { x_size: 500, y_size: 500 };
    let image_gtk: gtk::Image = spiral.generate_to_gtk();

    image_map.borrow_mut().insert(1, image_gtk);

    let window_weak = window.downgrade();

    button.connect_clicked(clone!(box_vert, image_map => move |_| {
        let window = match window_weak.upgrade() {
            Some(window) => window,
            None => return
        };
        box_vert.remove(image_map.borrow().get(&1).unwrap());

        let image_gtk: gtk::Image = spiral.generate_to_gtk();
        image_map.borrow_mut().insert(1, image_gtk);
        box_vert.pack_start(image_map.borrow().get(&1).unwrap(), false, false, 20);
        window.show_all();
    }));

    box_vert.pack_start(image_map.borrow().get(&1).unwrap(), false, false, 20);
    box_vert.pack_end(&button, false, false, 5);

    window.add(&box_vert);
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
