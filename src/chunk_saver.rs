use crate::andy_vectors::Vec2D;
use crate::image_reader;
use crate::image_splitter::FloodedChunkInfo;
use core::iter::zip;

fn isolate_chunk(
    chunk: &FloodedChunkInfo,
    flooded_mask: &Vec2D<Option<u32>>,
    pixels: &Vec2D<(u8, u8, u8, u8)>,
) -> Vec2D<(u8, u8, u8, u8)> {
    let chunk_pixels: Vec<(u8, u8, u8, u8)> = zip(pixels.iter_rows(), flooded_mask.iter_rows())
        .enumerate()
        .filter(|(curr_y, _)| (*curr_y >= chunk.top_left.1 && *curr_y < chunk.bot_right.1))
        .flat_map(|(_curr_y, (pix_row, flood_row))| {
            zip(pix_row, flood_row)
                .enumerate()
                .filter(|(curr_x, _)| (*curr_x >= chunk.top_left.0 && *curr_x < chunk.bot_right.0))
                .map(|(_curr_x, (pix, flood_val))| {
                    let is_pixel_part_of_chunk: bool = match flood_val {
                        None => false,
                        Some(x) => *x == chunk.val,
                    };
                    if is_pixel_part_of_chunk {
                        *pix
                    } else {
                        (255, 0, 255, 255)
                    }
                })
                .collect::<Vec<(u8, u8, u8, u8)>>()
        })
        .collect();

    let w: usize = chunk.bot_right.0 - chunk.top_left.0;
    let h: usize = chunk.bot_right.1 - chunk.top_left.1;
    Vec2D {
        data: chunk_pixels,
        w,
        h,
    }
}

fn save_chunk(chunk_image: Vec2D<(u8, u8, u8, u8)>, filename: String) {
    let chunk_image_output = image_reader::make_image_from_vec(chunk_image);
    chunk_image_output.save(filename).unwrap();
}

pub fn save_chunks(
    chunks_to_save: Vec<&FloodedChunkInfo>,
    flooded_mask: &Vec2D<Option<u32>>,
    pixels: &Vec2D<(u8, u8, u8, u8)>,
) {
    for (number, chunk) in chunks_to_save.iter().enumerate() {
        let chunk_image: Vec2D<(u8, u8, u8, u8)> = isolate_chunk(chunk, flooded_mask, pixels);
        let filename: String = format!("chunks/chunks{}.png", number);
        save_chunk(chunk_image, filename);
    }
}
