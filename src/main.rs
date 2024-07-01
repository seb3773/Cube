// Cube rotation code in rust.
pub mod vec; 
use crate::vec::{Vector, Rotation};
extern crate termion;
use std::thread;
use std::time::Duration;
use std::env;
use std::io::{stdout, Write};
use termion::terminal_size;

// cube parameters.
const BG_CHAR: char = ' '; 
const CUBE_DISTANCE: f32 = 60.; // K2
const SCREEN_DISTANCE: f32 = 40.; // K1
// angles of rotation on each axis.
const X: f32 = 0.;
const Y: f32 = 0.;
const Z: f32 = 0.;
// Default frame delay
const DEFAULT_FRAME_DELAY: u64 = 50;

// Couleurs ANSI
const COLORS: [&str; 6] = [
    "\x1b[31m", // Rouge
    "\x1b[32m", // Vert
    "\x1b[33m", // Jaune
    "\x1b[34m", // Bleu
    "\x1b[35m", // Magenta
    "\x1b[36m", // Cyan
];
const RESET_COLOR: &str = "\x1b[0m";

fn parse_surface(
    mut origin_vector: Vector,
    angles: [f32; 3],
    ch: char, 
    color: &str,
    z_buffer: &mut Vec<Vec<i32>>,
    output_buffer: &mut Vec<Vec<String>>,
    width: usize,
    height: usize
) 
{
    let x_theta = angles[0];
    let y_theta = angles[1];
    let z_theta = angles[2];
    // rotate the vector.
    origin_vector.rotate_all(x_theta, y_theta, z_theta);  
    let w_offset = width as f32 / 2.;
    let h_offset = height as f32 / 2.;
    // Calc one over Z.
    let ooz: f32 = 1.0 / (CUBE_DISTANCE + origin_vector.z);
    // xp is multiplied by 2. since the width of any char is smaller than its height.
    let xp: usize = (w_offset + SCREEN_DISTANCE * ooz * origin_vector.x * 2.) as usize; 
    let yp: usize = (h_offset + SCREEN_DISTANCE * ooz * origin_vector.y) as usize;
    if xp >= width || yp >= height { 
        return 
    };
    if ooz > z_buffer[yp][xp] as f32 { 
        // Update the Z-buffer and plot the point.
        z_buffer[yp][xp] = ooz as i32;
        output_buffer[yp][xp] = format!("{}{}{}", color, ch, RESET_COLOR);
    }
}

fn render_cube(frame_delay: u64) {
    let (width, height) = terminal_size().unwrap();
    let width = width as usize;
    let height = height as usize;
    
    // Calculate cube size based on terminal size
    let cube_len = (height.min(width / 2) as f32 * 0.25) as i32;

    let mut output_buffer: Vec<Vec<String>> = vec![vec![BG_CHAR.to_string(); width]; height]; // Output on the screen.
    let mut depth_checker: Vec<Vec<i32>> = vec![vec![0; width]; height]; // Z buffer.
    let mut rotation_angles: [f32; 3] = [X, Y, Z];
    // Origin Vectors.
    let mut v1: Vector;
    let mut v2: Vector;
    let mut v3: Vector;
    let mut v4: Vector;
    let mut v5: Vector;
    let mut v6: Vector;
    loop {
        // Parse all 6 sides of the cube into the buffer.
        for cube_x in -cube_len..cube_len {
            for cube_y in -cube_len..cube_len {
                v1 = Vector {x:    cube_x as f32, y:    cube_y as f32, z: -cube_len as f32 };
                v2 = Vector {x:  cube_len as f32, y:    cube_y as f32, z:    cube_x as f32 };
                v3 = Vector {x: -cube_len as f32, y:    cube_y as f32, z:   -cube_x as f32 };
                v4 = Vector {x:   -cube_x as f32, y:    cube_y as f32, z:  cube_len as f32 };
                v5 = Vector {x:    cube_x as f32, y: -cube_len as f32, z:   -cube_y as f32 };
                v6 = Vector {x:    cube_x as f32, y:  cube_len as f32, z:    cube_y as f32 };
                parse_surface(v1, rotation_angles, '$', COLORS[0], &mut depth_checker, &mut output_buffer, width, height);
                parse_surface(v2, rotation_angles, '!', COLORS[1], &mut depth_checker, &mut output_buffer, width, height);
                parse_surface(v3, rotation_angles, '~', COLORS[2], &mut depth_checker, &mut output_buffer, width, height);
                parse_surface(v4, rotation_angles, '+', COLORS[3], &mut depth_checker, &mut output_buffer, width, height);
                parse_surface(v5, rotation_angles, '@', COLORS[4], &mut depth_checker, &mut output_buffer, width, height);
                parse_surface(v6, rotation_angles, '.', COLORS[5], &mut depth_checker, &mut output_buffer, width, height);
            }
        }
        // Plot the buffer.
        for row in &output_buffer {
            for px in row { 
                print!("{}", px);
            }
            println!();
        }
        println!("\x1b[H"); // Return to the HOME offset in the terminal.
        // Inc the angles.
        rotation_angles[0] += 0.05;
        rotation_angles[1] += 0.05;
        rotation_angles[2] += 0.05;
        // Renew buffers. 
        output_buffer = vec![vec![BG_CHAR.to_string(); width]; height]; 
        depth_checker = vec![vec![0; width]; height];
        thread::sleep(Duration::from_millis(frame_delay));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let frame_delay = if args.len() > 1 {
        args[1].parse().unwrap_or(DEFAULT_FRAME_DELAY)
    } else {
        DEFAULT_FRAME_DELAY
    };

    // Start animation.
    println!("\x1b[2J\r\x1b[H");
    render_cube(frame_delay);
}
