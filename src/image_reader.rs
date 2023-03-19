use super::andy_vectors::Vec2D as Vec2D;
use image::io::Reader as ImageReader;

fn read_image(filename: &str) -> image::RgbaImage{
    let img = ImageReader::open(filename)
        .expect(&("failed to open image ".to_owned() + filename))
        .decode()
        .expect(&("failed to decode image ".to_owned() + filename))
        .into_rgba8();
    return img;
}

pub fn read_image_into_vec(filename: &str) -> Vec2D<(u8, u8, u8, u8)>{
    let img = read_image(filename);

    let img_bytes: Vec<u8> = img.as_raw().to_vec();
    let (w, h) = img.dimensions();

    let num_bytes = img_bytes.len();
    assert!(num_bytes == (4*w*h).try_into().unwrap());

    let pixel_arr: Vec<(u8, u8, u8, u8)> = (0..num_bytes)
        .filter(|x| x % 4 == 0)
        .map(|x| (img_bytes[x], img_bytes[x+1], img_bytes[x+2], img_bytes[x+3]))
        .collect();

    return Vec2D::new(pixel_arr, w.try_into().unwrap(), h.try_into().unwrap());
}

pub fn make_image_from_vec(data: Vec2D<(u8, u8, u8, u8)>) -> image::RgbaImage{
    let data_flattened: Vec<u8> = data.data
        .iter()
        .flat_map(|tup| [tup.0, tup.1, tup.2, tup.3])
        .collect();
    assert!(data_flattened.len() == (data.h * data.w * 4));
    return image::ImageBuffer::from_raw(data.w.try_into().unwrap(), data.h.try_into().unwrap(), data_flattened)
        .unwrap();
}
