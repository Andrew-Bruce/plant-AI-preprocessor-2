//https://en.wikipedia.org/wiki/HSL_and_HSV#From_RGB
fn rgb_to_hsv(rgb: (i32, i32, i32)) -> (i32, i32, i32){
    let r: f32 = (rgb.0 as f32)/255.0;
    let g: f32 = (rgb.1 as f32)/255.0;
    let b: f32 = (rgb.2 as f32)/255.0;
    let v: f32 = r.max(g).max(b);
    
    let c: f32 = v - r.min(g).min(b);

    let h: f32 =
        if c == 0.0 {
            0.0
        }else if v == r {
            60.0*(0.0+((g-b)/c))
        }else if v == g {
            60.0*(2.0+((b-r)/c))
        }else if v == b {
            60.0*(4.0+((r-g)/c))
        }else{
            unreachable!();
        };
    let s: f32 = if v == 0.0 {0.0} else {c/v};

    return (h as i32, (s*255.0) as i32, (v*255.0) as i32);
}


pub fn pixel_green_enough(rgba_pix: (u8, u8, u8, u8)) -> bool{
    let r: i32 = rgba_pix.0 as i32;
    let g: i32 = rgba_pix.1 as i32;
    let b: i32 = rgba_pix.2 as i32;

    let (h, _s, _v) = rgb_to_hsv((r, g, b));

    let how_far_from_green:i32 = (h - 120).abs();
    return how_far_from_green < 80;
}
