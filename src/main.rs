#![allow(unused_variables, unused_imports)]
use image::{io::Reader as ImageReader, ColorType, GenericImageView, Primitive, Rgb};
use std::{error::Error, ops::AddAssign, usize};

#[derive(Debug)]
struct RGB(i32, i32, i32);

impl RGB {
    fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB(r as i32, g as i32, b as i32)
    }
    fn new_hex(val: u32) -> RGB {
        // println!("{:f}", );
        println!("{:#b}", val);
        let r = (val & 0x00FF0000) >> 16;
        let g = (val & 0x0000FF00) >> 8;
        let b = val & 0x000000FF;
        println!("{:#b}", r >> 16);
        println!("{:#b}", g >> 8);
        println!("{:#b}", b);

        RGB(r as i32, g as i32, b as i32)
    }
}

fn get_total_diff(c1: &RGB, c2: &Rgb<u8>) -> i32 {
    let mut diff = 0;
    diff += (c1.0 - c2.0[0] as i32).abs();
    diff += (c1.1 - c2.0[1] as i32).abs();
    diff += (c1.2 - c2.0[2] as i32).abs();

    diff
}
fn get_avg_diff(c1: &RGB, c2: &Rgb<u8>) -> i32 {
    get_total_diff(c1, c2) / 3
}

fn get_sp_diff(c1: &RGB, c2: &Rgb<u8>) -> i32 {
   get_avg_diff(c1, c2) + ((c2.0[0] as i32 - c2.0[1] as i32).abs() - c2.0[1] as i32).abs()
}

fn set_least_diff(og: &mut Rgb<u8>, colors: &Vec<RGB>) {
    if colors.len() == 0 {
        return;
    }

    let mut sml_diff = get_sp_diff(&colors[0], og);

    for c in colors {
        let diff = get_sp_diff(c, og);
        if diff <= sml_diff {
            sml_diff = diff;
        }
    }

    for c in colors {
        let diff = get_sp_diff(c, og);
        if diff == sml_diff {
            og.0[0] = c.0 as u8;
            og.0[1] = c.1 as u8;
            og.0[2] = c.2 as u8;
            break;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut image = ImageReader::open("Fuzzy.jpg")?.decode()?;
    let dimensions = { image.dimensions() };
    let colors = vec![
        RGB::new(255, 255, 255),
        RGB::new(0, 0, 0),
        RGB::new_hex(0xBC2125),
        RGB::new_hex(0x00B17136),
        RGB::new_hex(0x492E19),
        // RGB::new_hex(0xBC2125),


        // RGB::new_hex(0xA99F6A),
        RGB::new_hex(0x3D6A7F),
        // RGB::new_hex(0xAB4F42),
        // RGB::new(255,0, 0),
        // RGB::new(0, 255, 0),
        // RGB::new(0, 0, 255),
    ];
    println!("{:?}", colors);
    let pixels = image.as_mut_rgb8().unwrap();
    {
        for x in 0..dimensions.0 {
            for y in 0..dimensions.1 {
                set_least_diff(&mut pixels.get_pixel_mut(x, y), &colors);
            }
        }
    }

    pixels.save("image.jpg")?;

    Ok(())
}
