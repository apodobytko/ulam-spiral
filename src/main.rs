extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate image;

mod spiral;
mod front;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::PathBuf;
use std::env::args;

use self::gio::prelude::*;
use self::gtk::prelude::*;

use gtk::WindowPosition;

use self::spiral::Spiral;
use self::front::SaveDialog;

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
    window.set_default_size(400, 400);
    window
}

fn generate_image(adj_x: &gtk::Adjustment, adj_y: &gtk::Adjustment) -> gtk::Image {
    let x_size: u32 = adj_x.get_value() as u32;
    let y_size: u32 = adj_y.get_value() as u32;
    let spiral = Spiral { x_size, y_size };
    spiral.generate_to_gtk()
}

fn save_image(path: PathBuf, adj_x: &gtk::Adjustment, adj_y: &gtk::Adjustment) {
    let x_size: u32 = adj_x.get_value() as u32;
    let y_size: u32 = adj_y.get_value() as u32;
    let spiral = Spiral { x_size, y_size };
    let image = spiral.generate();
    match image.save(path) {
        Ok(_) => println!("Ok, saved!"),
        Err(e) => println!("{}", e)
    }
}

fn build_ui(app: &gtk::Application) {
    let window = create_main_window(app);

    let image_map: Rc<RefCell<HashMap<usize, gtk::Image>>> = Rc::new(RefCell::new(HashMap::new()));
    let box_vert = gtk::Box::new(gtk::Orientation::Vertical, 50);
    let box_horiz = gtk::Box::new(gtk::Orientation::Horizontal, 50);
    let button = gtk::Button::new_with_label("Generate spiral");

    let adj_x = gtk::Adjustment::new(200.0, 10.0, 1000.0, 2.0, 0.0, 0.0);
    let adj_y = gtk::Adjustment::new(400.0, 10.0, 1000.0, 2.0, 0.0, 0.0);
    let spin_button_x = gtk::SpinButton::new(&adj_x, 2.0, 0);
    let spin_button_y = gtk::SpinButton::new(&adj_y, 2.0, 0);
    let save_button = gtk::Button::new_with_label("Save image");

    let image_gtk = generate_image(&adj_x, &adj_y);

    box_horiz.pack_start(&spin_button_x, false, false, 20);
    box_horiz.pack_start(&spin_button_y, false, false, 20);
    box_horiz.pack_end(&button, false, false, 20);
    box_horiz.pack_end(&save_button, false, false, 20);

    image_map.borrow_mut().insert(1, image_gtk);

    let window_weak = window.downgrade();

    button.connect_clicked(clone!(box_vert, image_map, adj_x, adj_y => move |_| {
        let window = match window_weak.upgrade() {
            Some(window) => window,
            None => return
        };
        box_vert.remove(image_map.borrow().get(&1).unwrap());

        let image_gtk = generate_image(&adj_x, &adj_y);

        image_map.borrow_mut().insert(1, image_gtk);
        box_vert.pack_start(image_map.borrow().get(&1).unwrap(), false, false, 20);
        window.show_all();
    }));

    save_button.connect_clicked(clone!(adj_x, adj_y => move |_| {
        let save_dialog = SaveDialog::new();
        match save_dialog.get_user_choice() {
            Some(path) => {
                save_image(path, &adj_x, &adj_y);
            },
            None => { println!("srsly decide what u want to do"); return }
        };

    }));

    box_vert.pack_start(image_map.borrow().get(&1).unwrap(), false, false, 20);
    box_vert.pack_end(&box_horiz, false, false, 20);

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
