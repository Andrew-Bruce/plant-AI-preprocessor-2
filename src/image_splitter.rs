use crate::andy_vectors::Vec2D;
use std::cmp;
use std::collections::VecDeque;

pub struct FloodedChunkInfo {
    pub val: u32,
    pub num_pixels: u64,
    pub top_left: (usize, usize),
    pub bot_right: (usize, usize),
}

//BFS is better than DFS cause doing it DFS with recursive causes a stackoverflow for large chunks, using a
//queue and a loop avoids too many stack frames
fn flood_from_pixel_bfs(
    mask: &Vec2D<bool>,
    start_pos: (usize, usize),
    value: u32,
    output: &mut Vec2D<Option<u32>>,
) -> FloodedChunkInfo {
    assert!(mask.w == output.w && mask.h == output.h);

    let mut to_search: VecDeque<(usize, usize)> = VecDeque::new();
    to_search.push_back(start_pos);

    let mut top_left_x: usize = start_pos.0;
    let mut top_left_y: usize = start_pos.1;
    let mut bot_right_x: usize = start_pos.0;
    let mut bot_right_y: usize = start_pos.1;
    let mut num_pixels: u64 = 0;

    while !to_search.is_empty() {
        let pos: (usize, usize) = to_search.pop_front().unwrap();

        if mask[pos] {
            assert!(output[pos].is_none() || output[pos] == Some(value));

            if output[pos] == Some(value) {
                continue;
            }
            output[pos] = Some(value);
            num_pixels += 1;

            let x: usize = pos.0;
            let y: usize = pos.1;

            top_left_x = cmp::min(top_left_x, x);
            top_left_y = cmp::min(top_left_y, y);
            bot_right_x = cmp::max(bot_right_x, x);
            bot_right_y = cmp::max(bot_right_y, y);

            for dy in -1isize..=1 {
                for dx in (-1isize..=1).filter(|dx| {
                    mask.in_bounds(
                        dx + isize::try_from(x).unwrap(),
                        dy + isize::try_from(y).unwrap(),
                    )
                }) {
                    let new_pos: (usize, usize) = (
                        (isize::try_from(x).unwrap() + dx).try_into().unwrap(),
                        (isize::try_from(y).unwrap() + dy).try_into().unwrap(),
                    );
                    to_search.push_back(new_pos);
                }
            }
        } else {
            assert!(output[pos].is_none());
        }
    }

    FloodedChunkInfo {
        val: value,
        num_pixels,
        top_left: (top_left_x, top_left_y),
        bot_right: (bot_right_x, bot_right_y),
    }
}

pub fn flood_mask(mask: Vec2D<bool>) -> (Vec<FloodedChunkInfo>, Vec2D<Option<u32>>) {
    let mut output: Vec2D<Option<u32>> = Vec2D {
        data: vec![None; mask.w * mask.h],
        w: mask.w,
        h: mask.h,
    };

    let mut curr_val = 0;

    let mut chunks: Vec<FloodedChunkInfo> = vec![];

    for y in 0..mask.h {
        for x in 0..mask.w {
            let pos = (x, y);
            if mask[pos] && output[pos].is_none() {
                let chunk = flood_from_pixel_bfs(&mask, (x, y), curr_val, &mut output);
                chunks.push(chunk);
                curr_val += 1;
            }
        }
    }
    (chunks, output)
}
