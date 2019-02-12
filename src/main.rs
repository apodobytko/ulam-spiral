extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate image;

mod front;
mod spiral;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env::args;
use std::path::PathBuf;
use std::rc::Rc;

use self::gio::prelude::*;
use self::gtk::prelude::*;

use gtk::{Adjustment, Button, Label, RadioButton, SpinButton, WindowPosition};

use self::front::{ErrorDialog, SaveDialog};
use self::spiral::{Spiral, SpiralKind};

static INITIAL_SIDE_LEN: f64 = 500.0;

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

type ImageRef = Rc<RefCell<HashMap<usize, gtk::Image>>>;

#[derive(Clone)]
struct ImageMap {
    internal_value: ImageRef
}

impl ImageMap {
    fn new() -> ImageMap {
        ImageMap { internal_value: Rc::new(RefCell::new(HashMap::new())) }
    }

    fn get_image(&self) -> Result<gtk::Image, &str> {
        let image_map = self.internal_value.borrow();
        match image_map.get(&1) {
            Some(image) => Ok(image.clone()),
            None => Err("Expected an image!")
        }
    }

    fn set_image(&self, image_gtk: gtk::Image) {
        self.internal_value.borrow_mut().insert(1, image_gtk);
    }
}

fn create_main_window(app: &gtk::Application) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Ulam Spiral Generator");
    window.set_position(WindowPosition::Center);
    window.set_default_size(400, 400);
    window
}

fn generate_spiral(adj_x: &Adjustment) -> Rc<RefCell<Spiral>> {
    let x_size: u32 = adj_x.get_value() as u32;
    Rc::new(RefCell::new(Spiral::new(
        x_size,
        x_size,
        SpiralKind::Primes,
    )))
}

fn save_image(spiral: &Spiral, path: &PathBuf) {
    if let Some(extension) = path.extension() {
        match extension.to_str().expect("Failed to parse file extension.") {
            "png" | "jpeg" => {
                let image = spiral.generate();
                match image.save(&path) {
                    Ok(_) => println!("Ok, saved!"),
                    Err(e) => ErrorDialog::show(&format!("Sorry, failed to save the file. {}", e)),
                }
            }
            _ => {
                ErrorDialog::show("Sorry, only png and jpeg file formats are supported.");
            }
        }
    }
}

fn build_ui(app: &gtk::Application) -> Result<(), &str> {
    let window = create_main_window(app);

    // Instantiate the image container which will help us mutate the image from within the closure.
    let image_map = ImageMap::new();

    // Add all buttons and controls.
    let box_vert = gtk::Box::new(gtk::Orientation::Vertical, 20);
    let box_horiz = gtk::Box::new(gtk::Orientation::Horizontal, 20);

    let radio_primes = RadioButton::new_with_label("Prime numbers");
    let radio_random = RadioButton::new_with_label_from_widget(&radio_primes, "Random odd numbers");

    let generate_button = Button::new_with_label("Generate spiral");
    let adj_x = gtk::Adjustment::new(INITIAL_SIDE_LEN, 1.0, 1000.0, 1.0, 0.0, 0.0);
    let save_button = Button::new_with_label("Save image");

    let spiral: Rc<RefCell<Spiral>> = generate_spiral(&adj_x);
    let image_gtk: gtk::Image = spiral.borrow().generate_to_gtk();
    image_map.set_image(image_gtk);

    let box_l = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let box_r = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let label = Label::new(None);
    label.set_markup("<sup>Side size</sup>");

    // Add items to layout boxes.
    box_l.pack_start(&radio_primes, false, false, 2);
    box_l.pack_start(&radio_random, false, false, 2);
    box_l.pack_start(&SpinButton::new(&adj_x, 2.0, 0), false, false, 10);
    box_l.add(&label);

    box_r.pack_start(&generate_button, false, false, 10);
    box_r.pack_start(&save_button, false, false, 10);

    box_horiz.pack_start(&box_l, true, false, 50);
    box_horiz.pack_start(&box_r, true, false, 50);

    let window_weak = window.downgrade();

    // Bind action to the generate button.
    generate_button.connect_clicked(clone!(
            box_vert, image_map, spiral, adj_x, radio_primes => move |_| {

        let window = match window_weak.upgrade() {
            Some(window) => window,
            None => return
        };

        // Remove existing image from the hashmap.
        match image_map.get_image() {
            Ok(image) => box_vert.remove(&image),
            Err(e) => println!("{}", e),
        }
        spiral.borrow_mut().randomize_color();
        spiral.borrow_mut().set_size(adj_x.get_value() as u32);

        if radio_primes.get_active() {
            spiral.borrow_mut().set_kind(SpiralKind::Primes);
        } else {
            spiral.borrow_mut().set_kind(SpiralKind::Random);
        }
        let image_gtk: gtk::Image = spiral.borrow().generate_to_gtk();

        // Add newly generated image.
        image_map.set_image(image_gtk);

        match image_map.get_image() {
            Ok(image) => box_vert.pack_start(&image, false, false, 20),
            Err(e) => println!("{}", e),
        }
        //}
        window.show_all();
    }));

    save_button.connect_clicked(move |_| {
        let save_dialog = SaveDialog::new();
        match save_dialog.get_user_choice() {
            Some(path) => {
                save_image(&spiral.borrow(), &path);
            }
            None => return,
        };
    });

    match image_map.get_image() {
        Ok(image) => box_vert.pack_start(&image, false, false, 20),
        Err(e) => println!("{}", e),
    }
    box_vert.pack_end(&box_horiz, true, true, 20);

    window.add(&box_vert);

    window.connect_delete_event(|win, _| {
        win.destroy();
        Inhibit(false)
    });
    window.show_all();
    Ok(())
}

fn main() {
    let application = gtk::Application::new("com.ulam.spiral", gio::ApplicationFlags::empty())
        .expect("Initialization failed.");
    application.connect_startup(|app| {
        match build_ui(app) {
            Ok(_v) => (),
            Err(e) => println!("Failed to build the app: {:?}", e),
        }
    });
    application.run(&args().collect::<Vec<_>>());
}
