use image::Rgba;
use crate::rgb::RGB;

fn get_total_diff(c1: &RGB, c2: &Rgba<u8>) -> i32 {
    let mut diff = 0;
    diff += (c1.0 - c2.0[0] as i32).abs();
    diff += (c1.1 - c2.0[1] as i32).abs();
    diff += (c1.2 - c2.0[2] as i32).abs();

    diff
}
fn get_avg_diff(c1: &RGB, c2: &Rgba<u8>) -> i32 {
    get_total_diff(c1, c2) / 3
}

fn get_sp_diff(c1: &RGB, c2: &Rgba<u8>) -> i32 {
    get_avg_diff(c1, c2) + ((c2.0[0] as i32 - c2.0[1] as i32).abs() - c2.0[1] as i32).abs()
}

pub fn set_least_diff(og: &mut Rgba<u8>, colors: &Vec<RGB>) {
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
