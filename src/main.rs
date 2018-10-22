extern crate image;

mod sieve;

use self::sieve::generate_primes;

fn move_cursor(x: &mut u32, y: &mut u32, direction: &str) {
    let step = 3;
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

fn generate_spiral() {
    let mut img = image::ImageBuffer::new(200, 200);
    let pixel = image::Luma([200]);
    let mut stop = false;
    let mut x = 100;
    let mut y = 100;
    let mut times = 1;
    while !stop {

        img.put_pixel(x, y, pixel);
        while times < 10 {
            times += 1;
            for _ in 0..times {
                move_cursor(&mut x, &mut y, "up");
                img.put_pixel(x, y, pixel);
            }

            for _ in 0..times {
                move_cursor(&mut x, &mut y, "right");
                img.put_pixel(x, y, pixel);
            }

            times += 1;
            for _ in 0..times {
                move_cursor(&mut x, &mut y, "down");
                img.put_pixel(x, y, pixel);
            }

            for _ in 0..times {
                move_cursor(&mut x, &mut y, "left");
                img.put_pixel(x, y, pixel);
            }
        }
        stop = true;
    }
    img.save("image.png").unwrap();
}

fn main() {
    generate_spiral();
    generate_primes(10000);
}
