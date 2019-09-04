use std::fs::{File, create_dir};
use std::{io, fs};
use std::time::Instant;
use self::image::{FilterType, ImageFormat};
use std::io::Read;
use std::ptr::hash;

extern crate reqwest;
extern crate image;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::path::Path;

pub fn resize_image_from_url(url: &String) -> bool {
//    let target = "http://www.rust-lang.org/logos/rust-logo-512x512.png";
    let  t = Instant::now();
    let mut response = reqwest::get(url.as_str()).unwrap();
    println!("{:?}", response);

    let array: Vec<&str> = url.split(".").collect();
    let mut format = array[array.len()-1];
    println!("f4 {}", format);
    let filenameVec: Vec<&str> =  url.split("/").collect();
    let mut filename= filenameVec[filenameVec.len()-1];
    println!("{}", filename);

    let mut end_format = "temp/1.".to_owned();
    end_format+=format;

    fs::create_dir_all("/temp");
    let mut file = File::create(&end_format).expect("err");
    io::copy(&mut response, &mut file).expect("er");

    let mut f: ImageFormat = ImageFormat::JPEG;
    if format.contains("jp") { f=ImageFormat::JPEG   }
    if format.contains("png") { f=ImageFormat::PNG   }

    let img = image::open(&end_format);
    let r = img.unwrap().resize(100, 100, FilterType::CatmullRom);
    let mut out = File::create(filename).unwrap();
    let a = r.save(&mut out, f);
    println!("{:?}", response.headers().get("content-type"));


    fs::remove_file(&end_format);

    println!("{:?}", t.elapsed());

    true
}