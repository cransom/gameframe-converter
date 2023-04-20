use image::codecs::gif::{GifEncoder, Repeat};
use image::io::Reader as ImageReader;
use image::{Frame, GenericImageView};
use serde::Deserialize;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct GameFrameConfig {
    #[serde(default)]
    animation: Animation,
    #[serde(default)]
    translate: Translate,
}
impl Default for GameFrameConfig {
    fn default() -> GameFrameConfig {
        GameFrameConfig {
            animation: Animation {
                ..Default::default()
            },
            translate: Translate {
                ..Default::default()
            },
        }
    }
}

#[derive(Deserialize, Debug)]
struct Animation {
    #[serde(rename = "hold", default)]
    delay_ms: u32,
    #[serde(rename = "loop", default)]
    #[allow(dead_code)]
    looping: bool,
}
impl Default for Animation {
    fn default() -> Animation {
        Animation {
            delay_ms: 50,
            looping: true,
        }
    }
}

#[derive(Deserialize, Debug)]
struct Translate {
    #[serde(rename = "moveX", default)]
    move_x: u32,
    #[serde(rename = "moveY", default)]
    move_y: u32,
    #[allow(dead_code)]
    #[serde(rename = "loop", default)]
    looping: bool,
    #[allow(dead_code)]
    #[serde(rename = "loop", default)]
    panoff: bool,
}
impl Default for Translate {
    fn default() -> Translate {
        Translate {
            move_x: 0,
            move_y: 0,
            looping: true,
            panoff: true,
        }
    }
}

fn main() {
    let subj = std::env::args().nth(1).expect("Usage: gameframe-converter <path>");
    let config_loc = format!("{}/{}", subj, "config.ini");
    let config_path = Path::new(config_loc.as_str());
    let mut config = if config_path.exists() {
        let s = fs::read(config_path).unwrap();
        let config_string = String::from_utf8_lossy(&s);
        toml::from_str(&config_string).unwrap()
    } else {
        println!("No config.ini found. Using defaults.");
        GameFrameConfig {
            ..Default::default()
        }
    };

    let bmp_list = get_bmps(Path::new(subj.as_str()));
    let gif_buff = File::create(format!("{}.gif", subj)).unwrap();
    let mut gif = GifEncoder::new(gif_buff);
    gif.set_repeat(Repeat::Infinite).unwrap();

    if bmp_list.len() > 1 {
        for b in &bmp_list {
            let frame = Frame::from_parts(
                ImageReader::open(b.path())
                    .unwrap()
                    .decode()
                    .unwrap()
                    .into_rgba8(),
                0,
                0,
                image::Delay::from_numer_denom_ms(config.animation.delay_ms, 1),
            );
            gif.encode_frame(frame).unwrap();
        }
    } else {
        let image = ImageReader::open(bmp_list.first().unwrap().path())
            .unwrap()
            .decode()
            .unwrap();

        /* I'll try just a little bit and do the right thing if there's a wide/tall
         * image but no move_x or move_y.
         */
        if config.translate.move_x == 0 && config.translate.move_y == 0 {
            println!("This is awkward. There's a single image but no move commands in the config. Trying to do the right thing.");
            if image.width() > 16 {
                config.translate.move_x = 16;
            }
            if image.height() > 16 {
                config.translate.move_y = 16;
            }
        }

        let mut x_pos = 0;
        let mut y_pos = 0;

        while x_pos < image.width() && y_pos < image.height() {
            let partial_image = image.view(x_pos, y_pos, 16, 16);
            // what about bounds here?
            let frame = Frame::from_parts(
                partial_image.to_image(),
                x_pos,
                y_pos,
                image::Delay::from_numer_denom_ms(config.animation.delay_ms, 1),
            );
            gif.encode_frame(frame).unwrap();
            x_pos += config.translate.move_x;
            y_pos += config.translate.move_y;
        }
    }
}


fn get_bmps(path: &Path) -> Vec<DirEntry> {
    let mut images = vec![];
    let dir_list = fs::read_dir(path).unwrap();
    for entry in dir_list.flatten() {
        if entry
            .path()
            .extension()
            .unwrap_or(std::ffi::OsStr::new(""))
            .to_ascii_lowercase()
            == "bmp"
        {
            images.push(entry);
        }
    }
    images.sort_by(|a, b| alphanumeric_sort::compare_path(a.path(), b.path()));
    images
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn all_the_bitmaps() {
        assert_eq!(get_bmps(Path::new("tests/ordered")).len(), 6);
    }
    #[test]
    fn ordered_bitmaps() {
        assert_eq!(get_bmps(Path::new("tests/ordered")).first().unwrap().path(), Path::new("tests/ordered/0.bmp"));
        assert_eq!(get_bmps(Path::new("tests/ordered")).last().unwrap().path(), Path::new("tests/ordered/10.bmp"));
    }
}
