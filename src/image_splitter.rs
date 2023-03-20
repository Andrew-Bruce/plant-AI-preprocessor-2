use crate::andy_vectors::Vec2D;
use std::collections::VecDeque;

//BFS is better than DFS cause doing it recursive causes a stackoverflow for large chunks
fn flood_from_pixel_bfs(
    mask: &Vec2D<bool>,
    start_pos: (usize, usize),
    value: u32,
    output: &mut Vec2D<Option<u32>>,
) {
    assert!(mask.w == output.w && mask.h == output.h);

    let mut to_search: VecDeque<(usize, usize)> = VecDeque::new();
    to_search.push_back(start_pos);

    while !to_search.is_empty() {
        let pos: (usize, usize) = to_search.pop_front().unwrap();

        if mask[pos] {
            assert!(output[pos].is_none() || output[pos] == Some(value));

            if output[pos] == Some(value) {
                continue;
            }
            output[pos] = Some(value);

            let x: usize = pos.0;
            let y: usize = pos.1;

            for dy in -1i64..=1 {
                for dx in -1i64..=1 {
                    let new_x: i64 = <usize as TryInto<i64>>::try_into(x).unwrap() + dx;
                    let new_y: i64 = <usize as TryInto<i64>>::try_into(y).unwrap() + dy;
                    if new_x < 0 || new_y < 0 {
                        continue;
                    }
                    let new_pos: (usize, usize) =
                        (new_x.try_into().unwrap(), new_y.try_into().unwrap());
                    if !mask.is_in_range(new_pos.0, new_pos.1) {
                        continue;
                    }
                    to_search.push_back(new_pos);
                }
            }
        } else {
            assert!(output[pos].is_none());
        }
    }
}

pub fn flood_mask(mask: Vec2D<bool>) -> Vec2D<Option<u32>> {
    let mut output: Vec2D<Option<u32>> = Vec2D {
        data: vec![None; mask.w * mask.h],
        w: mask.w,
        h: mask.h,
    };

    let mut curr_val = 0;

    for y in 0..mask.h {
        for x in 0..mask.w {
            let pos = (x, y);
            if mask[pos] && output[pos].is_none() {
                flood_from_pixel_bfs(&mask, (x, y), curr_val, &mut output);
                curr_val += 1;
            }
        }
    }
    output
}
