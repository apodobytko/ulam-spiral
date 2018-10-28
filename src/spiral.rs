extern crate image;

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

pub fn generate_spiral(primes: Vec<u64>) {
    let mut img = image::ImageBuffer::new(1000, 1000);
    let white_pixel = image::Luma([255]);
    let mut x = 500;
    let mut y = 500;
    let mut counter = 0;
    let mut times = 0;
    let primes_len = primes.len();
    let mut stop = false;

    img.put_pixel(x, y, white_pixel);

    while !stop {
        times += 1;
        for _ in 0..times {
            if (counter < primes_len) & (x > 2) {
                move_cursor(&mut x, &mut y, "up");
                if let 1 = primes[counter] { img.put_pixel(x, y, white_pixel) }
                counter += 1;
            }
        }

        for _ in 0..times {
            if (counter < primes_len) & (x > 2) {
                move_cursor(&mut x, &mut y, "right");
                if let 1 = primes[counter] { img.put_pixel(x, y, white_pixel) }
                counter += 1;
            }
        }

        times += 1;
        for _ in 0..times {
            if (counter < primes_len) & (x > 2) {
                move_cursor(&mut x, &mut y, "down");
                if let 1 = primes[counter] { img.put_pixel(x, y, white_pixel) }
                counter += 1;
            }
        }

        for _ in 0..times {
            if (counter < primes_len) & (x > 2) {
                move_cursor(&mut x, &mut y, "left");
                if let 1 = primes[counter] { img.put_pixel(x, y, white_pixel) }
                counter += 1;
            }
        }
        if (counter >= primes_len) | (x <= 2) {
            stop = true;
        }
    }
    img.save("image.png").unwrap();
}
