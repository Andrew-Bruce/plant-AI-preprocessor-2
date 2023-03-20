mod andy_vectors;
mod green_masking;
mod image_reader;
mod image_splitter;
use crate::andy_vectors::Vec2D;
use std::iter::zip;
fn main() {
    let pixels: Vec2D<(u8, u8, u8, u8)> = image_reader::read_image_into_vec("plant4.jpg");

    let masked_vec: Vec<bool> = pixels
        .data
        .iter()
        .map(|&x| green_masking::pixel_green_enough(x))
        .collect();

    let image_mask: Vec2D<bool> = Vec2D {
        data: masked_vec,
        w: pixels.w,
        h: pixels.h,
    };

    let masked_image: Vec2D<(u8, u8, u8, u8)> = Vec2D {
        data: zip(pixels.data.iter(), image_mask.data.iter())
            .map(|(&pixel, &is_masked)| if is_masked { pixel } else { (255, 0, 255, 255) })
            .collect(),
        w: pixels.w,
        h: pixels.h,
    };

    let masked_image_output = image_reader::make_image_from_vec(masked_image);
    masked_image_output.save("out.png").unwrap();



    let (_chunk_infos, flooded_mask) = image_splitter::flood_mask(image_mask);

    let colors: [(u8, u8, u8, u8); 6] = [
            (255, 0, 0, 255),
            (0, 255, 0, 255),
            (0, 0, 255, 255),
            (255, 255, 0, 255),
            (0, 255, 255, 255),
            (255, 0, 255, 255),
    ];

    let chunk_colors: Vec2D<(u8, u8, u8, u8)> = Vec2D {
        data: flooded_mask.data.iter()
            .map(|&flood_val| match flood_val{
                None => (0, 0, 0, 0),
                Some(val) => colors[(val%6) as usize],
            })
            .collect(),
        w: pixels.w,
        h: pixels.h,
    };

    let chunk_colors_image_output = image_reader::make_image_from_vec(chunk_colors);
    chunk_colors_image_output.save("chunks.png").unwrap();


}
