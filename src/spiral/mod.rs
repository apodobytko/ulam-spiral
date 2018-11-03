extern crate rand;
extern crate image;
extern crate gtk;
extern crate gdk_pixbuf;

use gdk_pixbuf::{Colorspace, Pixbuf};
use image::GenericImageView;


mod sieve;

pub struct Spiral {
    pub x_size: u32,
    pub y_size: u32,
}


impl Spiral {

    pub fn generate_to_gtk(&self) -> gtk::Image {
        let image_vec = self.generate();


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
        gtk::Image::new_from_pixbuf(&pixbuff)

    }

    pub fn generate(&self) -> Vec<u8> {
        let primes = sieve::generate_primes(100_000);
        let mut img = image::ImageBuffer::new(self.x_size, self.y_size);
        let (red, green, blue) = Spiral::random_colors();
        let pixel = image::Rgb([red, green, blue]);
        let mut x = self.x_size / 2;
        let mut y = self.y_size / 2;
        let mut counter = 0;
        let mut times = 0;
        let primes_len = primes.len();
        let mut stop = false;

        img.put_pixel(x, y, pixel);

        while !stop {
            times += 1;
            for _ in 0..times {
                if (counter < primes_len) & (x > 2) {
                    Spiral::move_cursor(&mut x, &mut y, "up");
                    if let 1 = primes[counter] { img.put_pixel(x, y, pixel) }
                    counter += 1;
                }
            }

            for _ in 0..times {
                if (counter < primes_len) & (x > 2) {
                    Spiral::move_cursor(&mut x, &mut y, "right");
                    if let 1 = primes[counter] { img.put_pixel(x, y, pixel) }
                    counter += 1;
                }
            }

            times += 1;
            for _ in 0..times {
                if (counter < primes_len) & (x > 2) {
                    Spiral::move_cursor(&mut x, &mut y, "down");
                    if let 1 = primes[counter] { img.put_pixel(x, y, pixel) }
                    counter += 1;
                }
            }

            for _ in 0..times {
                if (counter < primes_len) & (x > 2) {
                    Spiral::move_cursor(&mut x, &mut y, "left");
                    if let 1 = primes[counter] { img.put_pixel(x, y, pixel) }
                    counter += 1;
                }
            }
            if (counter >= primes_len) | (x <= 2) {
                stop = true;
            }
        }
        let mut buf = Vec::new();
        let dynamic_image = image::DynamicImage::ImageRgb8(img);
        dynamic_image.write_to(&mut buf, image::ImageOutputFormat::PNG)
                     .expect("Failed to write the buffer!");
        println!("Spiral generated.");
        buf
    }

    fn move_cursor(x: &mut u32, y: &mut u32, direction: &str) {
        let step = 2;
        if direction == "up" {
            *y -= step;
        } else if direction == "right" {
            *x += step;
        } else if direction == "down" {
            *y += step;
        } else if direction == "left" {
            *x -= step;
        } else {
            panic!("Choose something!")
        }
    }
  
    fn random_colors() -> (u8, u8, u8) {
        (rand::random(), rand::random(), rand::random())
    }

}



#[cfg(test)]
mod tests {

    use self::super::Spiral;

    #[test]
    fn test_move_cursor_up() {
        let mut x = 100;
        let mut y = 100;
        Spiral::move_cursor(&mut x, &mut y, "up");

        assert_eq!(x, 100);
        assert_eq!(y, 98);
    }

    #[test]
    fn test_move_cursor_right() {
        let mut x = 100;
        let mut y = 100;
        Spiral::move_cursor(&mut x, &mut y, "right");

        assert_eq!(x, 102);
        assert_eq!(y, 100);
    }

    #[test]
    fn test_move_cursor_down() {
        let mut x = 100;
        let mut y = 100;
        Spiral::move_cursor(&mut x, &mut y, "down");

        assert_eq!(x, 100);
        assert_eq!(y, 102);
    }

    #[test]
    fn test_move_cursor_left() {
        let mut x = 100;
        let mut y = 100;
        Spiral::move_cursor(&mut x, &mut y, "left");

        assert_eq!(x, 98);
        assert_eq!(y, 100);
    }

}