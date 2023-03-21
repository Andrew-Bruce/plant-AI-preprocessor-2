use crate::andy_vectors::Vec2D;

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
pub fn bloat_mask(mask: Vec2D<bool>, num_iters: u32) -> Vec2D<bool> {
    if num_iters == 0 {
        return mask;
    }

    bloat_mask(bloat_mask_one_iter(&mask), num_iters - 1)
}
