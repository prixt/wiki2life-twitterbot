use std::io::Write;
use std::fs::File;
use std::error::Error;

use rand::prelude::*;

mod cellular_automata;
mod giffer;
mod twitter;

fn main() -> Result<(), Box<dyn Error>> {
    let mut game_of_life = cellular_automata::CellularAutomata::new(500, 500);
    let mut rng = thread_rng();
    let data: Box<[bool]> = std::iter::repeat_with(|| rng.gen())
        .take(500*500)
        .collect();
    game_of_life.load(&data);
    let mut gif_maker = giffer::Giffer::new(500, 500, [255,255,255], [100,100,100]);

    gif_maker.add_frame(&data, 50);
    
    for _ in 0..60 {
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
    let file_name = utc_time.format("GIF-%Y-%b-%d--%H-%M-%S(UTC).gif").to_string();
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
    twitter::run(data, "Have a gif!")?;

    Ok(())
}
