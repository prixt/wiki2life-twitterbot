use rand::prelude::*;

pub fn pick_colors() -> ([u8;3], [u8;3]) {
    let mut rng = thread_rng();

    let (mut color1, mut color2) = if rng.gen::<f32>() >= 0.15 {
        let h1: f32 = rng.gen_range(0.0, 1.0);
        let s1: f32 = rng.gen_range(0.0, 10.0);
        let s1 = if s1 > 1.0 {1.0} else {s1};
        let v1: f32 = rng.gen_range(0.0, 5.0);
        let v1 = if v1 > 1.0 {1.0} else {v1};

        let h2: f32 = (h1 + rng.gen_range(0.1, 0.9)).fract();
        let s2: f32 = 1.0;
        let v2: f32 = 1.0;
        (hsv2rgb(h1,s1,v1), hsv2rgb(h2,s2,v2))
    } else {
        ([0,0,0], [255,255,255])
    };
    
    if rng.gen() {
        std::mem::swap(&mut color1, &mut color2);
    }
    (color1, color2)
}

fn hsv2rgb(h:f32, s:f32, v:f32) -> [u8;3] {
    if s == 0.0 {
        let v = (v * 255.0) as u8;
        return [v, v, v]
    }

    let h = h * 6.0;
    let (i, f) : (u32, f32) = (h as u32, h.fract());

    let (r,g,b) = match i {
        0 => (
            v,
            v * (1.0 - (s * (1.0 - f))),
            v * (1.0 - s)
        ),
        1 => (
            v * (1.0 - s),
            v,
            v * (1.0 - s * f)
        ),
        2 => (
            v * (1.0 - s * f),
            v,
            v * (1.0 - (s * (1.0 - f)))
        ),
        3 => (
            v * (1.0 - s * f),
            v * (1.0 - s),
            v
        ),
        4 => (
            v * (1.0 - (s * (1.0 - f))),
            v * (1.0 - s * f),
            v
        ),
        5 => (
            v,
            v * (1.0 - s * f),
            v * (1.0 - s)
        ),
        _ => unreachable!()
    };

    [(r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8]
}