mod andy_vectors;
mod green_masking;
mod image_reader;
mod image_splitter;
use crate::andy_vectors::Vec2D;
use std::iter::zip;

const SQUARE_COLOR: (u8, u8, u8, u8) = (255, 0, 0, 255);

fn draw_square(
    img: &mut Vec2D<(u8, u8, u8, u8)>,
    top_left: (usize, usize),
    bot_right: (usize, usize),
) {
    for x in top_left.0..bot_right.0 {
        img[(x, top_left.1)] = SQUARE_COLOR;
        img[(x, bot_right.1)] = SQUARE_COLOR;
    }
    for y in top_left.1..bot_right.1 {
        img[(top_left.0, y)] = SQUARE_COLOR;
        img[(bot_right.0, y)] = SQUARE_COLOR;
    }
}

fn bloat_mask_one_iter(mask: &Vec2D<bool>) -> Vec2D<bool> {
    let new_mask: Vec<bool> = (0..mask.h)
        .flat_map(|y| {
            (0..mask.w)
                .map(|x| {
                    let num_neighbors: u8 = (-1isize..=1)
                        .map(|dy| {
                            (-1isize..=1)
                                .filter(|dx| {
                                    mask.in_bounds(
                                        dx + isize::try_from(x).unwrap(),
                                        dy + isize::try_from(y).unwrap(),
                                    )
                                })
                                .map(|dx| {
                                    let check_pos: (usize, usize) = (
                                        (isize::try_from(x).unwrap() + dx).try_into().unwrap(),
                                        (isize::try_from(y).unwrap() + dy).try_into().unwrap(),
                                    );

                                    match mask[check_pos] {
                                        true => 1u8,
                                        false => 0u8,
                                    }
                                })
                                .sum::<u8>()
                        })
                        .sum();
                    num_neighbors > 4 || mask[(x, y)]
                })
                .collect::<Vec<bool>>()
        })
        .collect();

    Vec2D {
        data: new_mask,
        w: mask.w,
        h: mask.h,
    }
}
fn bloat_mask(mask: Vec2D<bool>, num_iters: u32) -> Vec2D<bool> {
    if num_iters == 0 {
        return mask;
    }

    bloat_mask(bloat_mask_one_iter(&mask), num_iters - 1)
}
fn main() {
    let pixels: Vec2D<(u8, u8, u8, u8)> = image_reader::read_image_into_vec("plant4.jpg");

    let image_mask: Vec2D<bool> = bloat_mask(
        Vec2D {
            data: pixels
                .data
                .iter()
                .map(|&x| green_masking::pixel_green_enough(x))
                .collect(),
            w: pixels.w,
            h: pixels.h,
        },
        20,
    );

    let mut masked_image: Vec2D<(u8, u8, u8, u8)> = Vec2D {
        data: zip(pixels.data.iter(), image_mask.data.iter())
            .map(|(&pixel, &is_masked)| if is_masked { pixel } else { (0, 0, 0, 255) })
            .collect(),
        w: pixels.w,
        h: pixels.h,
    };

    let (chunks, flooded_mask) = image_splitter::flood_mask(image_mask);

    let colors: [(u8, u8, u8, u8); 6] = [
        (255, 0, 0, 255),
        (0, 255, 0, 255),
        (0, 0, 255, 255),
        (255, 255, 0, 255),
        (0, 255, 255, 255),
        (255, 0, 255, 255),
    ];

    let chunk_colors: Vec2D<(u8, u8, u8, u8)> = Vec2D {
        data: flooded_mask
            .data
            .iter()
            .map(|&flood_val| match flood_val {
                None => (0, 0, 0, 255),
                Some(val) => colors[(val % 6) as usize],
            })
            .collect(),
        w: pixels.w,
        h: pixels.h,
    };

    for chunk in chunks.iter().filter(|info| info.num_pixels > 200) {
        draw_square(&mut masked_image, chunk.top_left, chunk.bot_right);
    }

    let chunk_colors_image_output = image_reader::make_image_from_vec(chunk_colors);
    chunk_colors_image_output.save("chunks.png").unwrap();
    let masked_image_output = image_reader::make_image_from_vec(masked_image);
    masked_image_output.save("out.png").unwrap();
}
