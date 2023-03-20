use crate::andy_vectors::Vec2D;
use std::cmp;
pub struct FloodChunkInfo {
    num_pixels: u64,
    top_left: (usize, usize),
    bot_right: (usize, usize),
    val: u32,
}

fn flood_from_pixel(
    mask: &Vec2D<bool>,
    pos: (usize, usize),
    value: u32,
    output: &mut Vec2D<Option<u32>>,
) -> FloodChunkInfo {
    assert!(output[pos].is_none() || output[pos] == Some(value));
    if !mask[pos] {
        return FloodChunkInfo {
            num_pixels: 0,
            top_left: pos,
            bot_right: pos,
            val: value,
        };
    }

    let mut num_flooded: u64 = 0;
    let mut top_left_x: usize = pos.0;
    let mut top_left_y: usize = pos.1;
    let mut bot_right_x: usize = pos.0;
    let mut bot_right_y: usize = pos.1;

    output[pos] = Some(value);
    num_flooded += 1;
    for dy in -1i64..=1 {
        for dx in -1i64..=1 {
            let x: usize = (<usize as TryInto<i64>>::try_into(pos.0).unwrap() + dx) as usize;
            let y: usize = (<usize as TryInto<i64>>::try_into(pos.1).unwrap() + dy) as usize;
            if mask.is_in_range(x, y) {
                let sub_chunk = flood_from_pixel(mask, (x, y), value, output);
                num_flooded += sub_chunk.num_pixels;
                top_left_x = cmp::min(top_left_x, sub_chunk.top_left.0);
                top_left_y = cmp::min(top_left_y, sub_chunk.top_left.1);
                bot_right_x = cmp::max(bot_right_x, sub_chunk.bot_right.0);
                bot_right_y = cmp::max(bot_right_y, sub_chunk.bot_right.1);
            }
        }
    }
    FloodChunkInfo {
        num_pixels: num_flooded,
        top_left: (top_left_x, top_left_y),
        bot_right: (bot_right_x, bot_right_y),
        val: value,
    }
}

pub fn flood_mask(mask: Vec2D<bool>) -> (Vec<FloodChunkInfo>, Vec2D<Option<u32>>){
    let mut output: Vec2D<Option<u32>> = Vec2D{
        data: vec![None; mask.w*mask.h],
        w: mask.w,
        h: mask.h,
    };

    let mut curr_val = 0;
    let mut chunks: Vec<FloodChunkInfo> = vec!();

    for y in 0..mask.h {
        for x in 0..mask.w {
            let pos = (x, y);
            if mask[pos] {
                let chunk: FloodChunkInfo = flood_from_pixel(&mask, (x, y), curr_val, &mut output);
                chunks.push(chunk);
                curr_val += 1;
            }
        }
    }
    (chunks, output)
}
