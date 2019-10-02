use rusttype::*;

const LINE_WIDTH_LIMIT: f32 = 400.0;

const TITLE_FONT_SIZE: f32 = 30.0;
const BODY_FONT_SIZE: f32 = 25.0;

const FILE_DATA: &[u8] = include_bytes!("../static/Silver.ttf");

pub fn generate_matrix(title: &str, content: &str) -> (usize, usize, Box<[bool]>) {
    let font = Font::from_bytes(FILE_DATA).unwrap();

    let title_scale = Scale{
        x: TITLE_FONT_SIZE, y: TITLE_FONT_SIZE,
    };
    let body_scale = Scale{
        x: BODY_FONT_SIZE, y: BODY_FONT_SIZE,
    };

    let title_vmetrics = font.v_metrics(title_scale);
    let body_vmetrics = font.v_metrics(body_scale);

    let title_offset = point(5.0, title_vmetrics.ascent + 5.0);
    let body_offset = point(
        5.0,
        title_vmetrics.ascent - title_vmetrics.descent
        + title_vmetrics.line_gap + body_vmetrics.ascent + 5.0
    );

    let title_glyphs = 
        font.layout(title, title_scale, title_offset);

    let h = body_vmetrics.ascent - body_vmetrics.descent + body_vmetrics.line_gap;
    let mut y = 0.0;
    let body_glyphs: Vec<PositionedGlyph<'_>> = font
        .glyphs_for(content.chars())
        .scan((None, 0.0), |(last, x), g| {
            let g = g.scaled(body_scale);
            if let Some(last) = last {
                *x += font.pair_kerning(body_scale, *last, g.id());
                if g.id() == '\n'.into_glyph_id(&font) || *x >= (LINE_WIDTH_LIMIT - 10.0) {
                    *x = 0.0;
                    y += h;
                }
            }
            let w = g.h_metrics().advance_width;
            let next = g.positioned(body_offset + vector(*x, y));
            last.replace(next.id());
            *x += w;
            Some(next)
        })
        .collect();
    let pixel_height = body_glyphs.last().unwrap()
        .pixel_bounding_box().unwrap()
        .max.y + 5;
    
    let mut data = vec![false; LINE_WIDTH_LIMIT as usize * pixel_height as usize];
    for g in title_glyphs.chain(body_glyphs.into_iter()) {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x,y,v| {
                if v >= 0.4 {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;
                    if x >= 0 && x < LINE_WIDTH_LIMIT as i32 && y >= 0 && y < pixel_height as i32 {
                        let x = x as usize;
                        let y = y as usize;
                        data[(x + y * LINE_WIDTH_LIMIT as usize)] = true;
                    }
                }
            })
        }
    }
    
    (LINE_WIDTH_LIMIT as usize, pixel_height as usize, data.into_boxed_slice())
}