use serde::Deserialize;
use std::fs;


#[derive(Deserialize,Debug)]
struct GameFrameConfig {
    animation: Animation,
    translate: Translate
}

#[derive(Deserialize,Debug)]
struct Animation {
    #[serde(rename = "hold")]
    delay_ms: u32,
    #[serde(rename = "loop")]
    looping: bool,
}

#[derive(Deserialize,Debug)]
struct Translate {
    #[serde(rename = "moveX")]
    move_x: u32,
    #[serde(rename = "moveY")]
    move_y: u32,
    #[serde(rename = "loop")]
    looping: bool,
    panoff: bool
}

fn main() {
    let config_bytes = fs::read("config.ini").unwrap();
    let config_string = String::from_utf8_lossy(config_bytes.as_ref());
    let config: GameFrameConfig = toml::from_str( config_string.as_ref() ).unwrap();
    println!("{:?}", config);

    // open dir, find bmp images, sort.

    let mut images = vec![];

    let mut paths: Vec<_> = fs::read_dir("example").unwrap()
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

    images.sort_by(|a,b| alphanumeric_sort::compare_path(a.path(),b.path()));

    for b in images {
        println!("{:?}", b.file_name());
    }

}
