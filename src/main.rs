extern crate minifb;
extern crate ufo_rs;

use minifb::{Window, Key, Scale, WindowOptions};

/// Import std stuff
use std::error::Error;
use std::time::Duration;
use std::thread;

// Import traits
use ufo_rs::traits::control::*;
use ufo_rs::traits::drone::*;

// Import controller
use ufo_rs::drones::jjrc::h61;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() -> Result<(), Box<dyn Error>> {
    let mut window = match Window::new("H61 Controller - Press ESC to exit", WIDTH, HEIGHT,
                                       WindowOptions {
                                           resize: true,
                                           scale: Scale::X2,
                                           ..WindowOptions::default()
                                       }) {
        Ok(win) => win,
        Err(err) => {
            panic!("Unable to create window {}", err);
        }

    };

    let mut buffer: Vec<u32> = Vec::with_capacity(WIDTH * HEIGHT);

  let mut size = (0, 0);

    // let mut noise;
    // let mut carry;
    // let mut seed = 0xbeefu32;


    // Create a new drone driver
    // TODO Fix bind error when not connected to drone
    let mut driver = h61::Driver::new();

    // Connect to drone
    let connection_res = driver.connect();

    match connection_res {
        Ok(_) => println!("Connected to drone."),
        Err(e) => panic!("Connection Error, check connection to drone"),
    };

    let mut flying: bool = false;

    for mut i in buffer.iter_mut() {
        i = &mut 0;
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
         {
            let new_size = window.get_size();
            if new_size != size {
                size = new_size;
                buffer.resize(size.0 * size.1 / 2 / 2, 0);
            }
        }

             // driver.request_video()?;
             // buffer = driver.read()?.iter().map(|item| *item as u32).collect();
//         for i in buffer.iter_mut() {
//             noise = seed;
//             noise >>= 3;
//             noise ^= seed;
//             carry = noise & 1;
//             noise >>= 1;
//             seed >>= 1;
//             seed |= carry << 30;
//             noise &= 0xFF;
//             *i = (noise << 16) | (noise << 8) | noise;
//         }

       // {
        //     let new_size = window.get_size();
        //     if new_size != size {
        //         size = new_size;
        //         buffer.resize(size.0 * size.1 / 2 / 2, 0);
        //     }
        // }

        window.get_keys().map(|keys| {
            for t in keys {
                // Handle input
                match t {
                    Key::Space => {
                        if flying {
                            flying = false;
                            driver.land().unwrap()
                        }
                        else {
                            flying = true;
                            driver.take_off().unwrap()
                        }
                    },
                    Key::I => driver.up(10).unwrap(),
                    Key::K => driver.down(10).unwrap(),
                    Key::J => driver.rot_left(1).unwrap(),
                    Key::L => driver.rot_right(1).unwrap(),
                    Key::U => driver.forwards(1).unwrap(),
                    Key::O => driver.backwards(1).unwrap(),
                    Key::H => driver.left(1).unwrap(),
                    Key::Semicolon => driver.right(1).unwrap(),
                    Key::S => driver.hover().unwrap(),
                    Key::C => driver.calibrate().unwrap(),
                    _ => (),
                }
            }
        });

        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer)?;
    }

    Ok(())
}
