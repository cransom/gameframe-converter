use image::codecs::gif::GifEncoder;
use image::codecs::gif::Repeat;
use image::io::Reader as ImageReader;
use image::{Frame, GenericImage};
use serde::Deserialize;
use std::fs;
use std::fs::DirEntry;
use std::fs::File;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct GameFrameConfig {
    animation: Animation,
    translate: Translate,
}

#[derive(Deserialize, Debug)]
struct Animation {
    #[serde(rename = "hold")]
    delay_ms: u32,
    #[serde(rename = "loop")]
    looping: bool,
}

#[derive(Deserialize, Debug)]
struct Translate {
    #[serde(rename = "moveX")]
    move_x: u32,
    #[serde(rename = "moveY")]
    move_y: u32,
    #[serde(rename = "loop")]
    looping: bool,
    panoff: bool,
}

fn get_bmps(path: &Path) -> Vec<DirEntry> {
    let mut images = vec![];

    let mut paths: Vec<_> = fs::read_dir("example")
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    let dir_list = fs::read_dir("example").unwrap();
    for entry in dir_list {
        if let Ok(entry) = entry {
            if entry.path().extension().unwrap().to_ascii_lowercase() == "bmp" {
                images.push(entry);
            }
        }
    }
    images.sort_by(|a, b| alphanumeric_sort::compare_path(a.path(), b.path()));
    return images;
}
fn main() {
    let config_bytes = fs::read("config.ini").unwrap();
    let config_string = String::from_utf8_lossy(config_bytes.as_ref());
    let config: GameFrameConfig = toml::from_str(config_string.as_ref()).unwrap();
    println!("{:?}", config);

    //let target_image = GenericImage::new();
    //

    let subj = "example";
    let bmp_list = get_bmps(Path::new(subj));

    let mut gif_buff = File::create(format!("{}.gif", subj)).unwrap();
    let mut gif = GifEncoder::new(gif_buff);

    if bmp_list.len() > 1 {
        println!("multiple images");
        for b in &bmp_list {
            //println!("{:?}", b.file_name());
            //let img = ImageReader::open(b.path()).expect("couldn't open _ file").decode();
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
        println!("single");
    }

    let first_image = &bmp_list.first();

    /*
    let mut file_out = File::open(format!("{}.gif", subj)).unwrap();
    let mut gif = GifEncoder::new(file_out);
    if config.animation.looping {
        gif.set_repeat(Repeat::Infinite);
    }
    */
}
