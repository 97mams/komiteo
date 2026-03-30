use std::io::Cursor;
use image::ImageReader;

 pub fn image(path: &str) {
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let img2 = ImageReader::new(Cursor::new(bytes)).with_guessed_format().unwrap().decode().unwrap();
    img.save("empty.jpg").unwrap();

let mut bytes: Vec<u8> = Vec::new();
img2.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png).unwrap();
}