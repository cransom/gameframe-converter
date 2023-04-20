use image::codecs::gif::{GifEncoder, Repeat};
//use image::codecs::gif::Repeat;
use image::io::Reader as ImageReader;
use image::{Frame,  GenericImageView};
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

fn main() {

    //let target_image = GenericImage::new();
    //

    //let subj = "example";
    let subj = std::env::args().nth(1).expect("no path given");
    println!("{}", subj);
    let bmp_list = get_bmps(Path::new(subj.as_str()));
    let config_loc  = format!("{}/{}", subj, "config.ini");
    let config_bytes = fs::read(Path::new( config_loc.as_str())).unwrap();
    let config_string = String::from_utf8_lossy(config_bytes.as_ref());
    let config: GameFrameConfig = toml::from_str(config_string.as_ref()).unwrap();
    println!("{}, {:?}", subj, config);

    let gif_buff = File::create(format!("{}.gif", subj)).unwrap();
    let mut gif = GifEncoder::new(gif_buff);
    gif.set_repeat(Repeat::Infinite).unwrap();

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

        let image = ImageReader::open(bmp_list.first().unwrap().path())
            .unwrap()
            .decode()
            .unwrap();

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
    //            println!("{} and {}", x_pos, y_pos);
        }
    }
}

fn get_bmps(path: &Path) -> Vec<DirEntry> {
    let mut images = vec![];
    let dir_list = fs::read_dir(path).unwrap();
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
