use std::io::Write;
use std::fs::File;
use std::error::Error;

use rand::prelude::*;

mod cellular_automata;
mod giffer;
mod twitter;
mod color_picker; use color_picker::pick_colors;

static GIF_W: usize = 300;
static GIF_H: usize = 300;
static MAX_FRAMES: usize = 500;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game_of_life = cellular_automata::CellularAutomata::new(GIF_W, GIF_H);
    let mut rng = thread_rng();
    let data: Box<[bool]> = std::iter::repeat_with(|| rng.gen())
        .take(GIF_W * GIF_H)
        .collect();
    game_of_life.load(&data);
    let (color1, color2) = pick_colors();
    let mut gif_maker = giffer::Giffer::new(GIF_W as u16, GIF_H as u16, color1, color2);

    gif_maker.add_frame(&data, 50);
    
    for _ in 0..(MAX_FRAMES/5) {
        game_of_life.step();
        game_of_life.step();
        game_of_life.step();
        game_of_life.step();
        game_of_life.step();
        if game_of_life.is_oscillating() {
            break
        }
    }
    for cells in game_of_life.get_previous_cells() {
        gif_maker.add_frame(cells, 10);
    }
    gif_maker.add_frame(game_of_life.get_current_cells(), 10);
    
    let utc_time = chrono::prelude::Utc::now();
    let file_name = utc_time.format("GIF%Y-%m-%d--%H-%M-%S(UTC).gif").to_string();
    let mut target_path = std::env::args().nth(1)
        .map_or_else(
            ||std::env::current_dir().unwrap(),
            |path_str| std::path::PathBuf::from(path_str)
        );
    assert!(target_path.is_dir(), "Target path is not a directory.");
    target_path.push(file_name);

    let data = gif_maker.encode()?;
    let mut file = File::create(target_path)?;
    file.write_all(&data)?;

    #[cfg(feature="twitter_ready")]
    twitter::run(data, utc_time.format("%B %e - %H:%M:%S (UTC)").to_string())?;

    Ok(())
}
