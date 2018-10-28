extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate gdk_pixbuf;
extern crate image;

mod sieve;
mod spiral;

use image::GenericImageView;
use std::env::args;
use self::gio::prelude::*;
use self::gtk::prelude::*;

use gtk::{WindowPosition};
use gdk_pixbuf::{Colorspace, Pixbuf};

use self::sieve::generate_primes;
use self::spiral::generate_spiral;

fn build_ui(app: &gtk::Application, image_vec: Vec<u8>) {
    let window = gtk::ApplicationWindow::new(app);
    window.set_title("Ulam Spiral Generator");
    window.set_position(WindowPosition::Center);
    window.set_default_size(1200, 1200);

    let image_parsed = image::load_from_memory(image_vec.as_slice()).unwrap();

    let pixbuff = Pixbuf::new_from_vec(
        image_parsed.raw_pixels(),
        Colorspace::Rgb,
        false,
        8,
        image_parsed.width() as i32,
        image_parsed.height() as i32,
        3 * image_parsed.width() as i32
    );

    let image_gtk = gtk::Image::new_from_pixbuf(&pixbuff);
    window.add(&image_gtk);

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
        let primes = generate_primes(100_000);
        let image = generate_spiral(primes);
        build_ui(app, image);
    });
    application.run(&args().collect::<Vec<_>>());

}
