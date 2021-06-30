use comparitors::set_least_diff;
pub use image::ImageBuffer;
use image::{io::Reader as ImageReader, GenericImageView, Rgba};
use std::error::Error;

mod comparitors;
mod rgb;

pub use rgb::{VecColor, ARGB, RGB};

pub fn png_to_pixels<S: AsRef<str>>(
    png_path: S,
    colors: Vec<RGB>,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Box<dyn Error>> {
    let mut image = ImageReader::open(png_path.as_ref())?.decode()?;
    let dimensions = { image.dimensions() };

    let pixels = match image.as_mut_rgba8() {
        Some(v) => v,
        None => return Err("could not convert to rgb8".into()),
    };
    {
        for x in 0..dimensions.0 {
            for y in 0..dimensions.1 {
                set_least_diff(&mut pixels.get_pixel_mut(x, y), &colors);
            }
        }
    }

    Ok(pixels.to_owned())
}
