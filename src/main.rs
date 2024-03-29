#[macro_use] extern crate log;

use std::io::Write;
use std::fs::File;

mod wiki;
mod typesetter;
mod cellular_automata;
mod giffer;
mod twitter;
mod color_picker; use color_picker::pick_colors;

static MAX_FRAMES: usize = 500;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting program.");
    let utc_time = chrono::prelude::Utc::now();

    info!("Reading wikipedia.");
    let (mut title, summary, url) = wiki::read_wiki()
        .map_err(|e| {
            error!("Failed while reading wikipedia! Error: {}", e);
            e
        })?;
    title += ":";
    info!("Random article:\nTitle: {}\nSummary: {}\nURL: {}", title, summary, url);
    info!("Rasterizing strings to bool slice.");
    let (w, h, data) = typesetter::generate_matrix(&title, &summary);
    
    info!("Creating CA from bool slice.");
    let mut game_of_life = cellular_automata::CellularAutomata::new(w, h);
    game_of_life.load(&data);

    info!("Picking two colors.");
    let (color1, color2) = pick_colors();

    info!("Building gif_maker.");
    let mut gif_maker = giffer::Giffer::new(w as u16, h as u16, color1, color2);

    info!("Setting first frame.");
    gif_maker.add_frame(&data, 100);
    
    info!("Stepping CA.");
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

    info!("Generating frames.");
    for cells in game_of_life.get_previous_cells() {
        gif_maker.add_frame(cells, 10);
    }
    gif_maker.add_frame(game_of_life.get_current_cells(), 10);

    info!("Generating file name and path.");
    let file_name = utc_time.format("GIF%Y-%m-%d--%H-%M-%S(UTC).gif").to_string();
    let mut target_path = std::env::args().nth(1)
        .map_or_else(
            ||std::env::current_dir().unwrap(),
            std::path::PathBuf::from
        );
    assert!(target_path.is_dir(), "Target path is not a directory.");
    target_path.push(file_name);

    info!("Encoding image to bytes.");
    let data = gif_maker.encode()
        .map_err(|e| {
            error!("Failed while encoding data! Error: {}", e);
            e
        })?;
    let mut file = File::create(target_path)
        .map_err(|e| {
            error!("Failed while creating gif file! Error: {}", e);
            e
        })?;
    
    info!("Saving image to file.");
    file.write_all(&data)
        .map_err(|e| {
            error!("Failed while writing data to gif file! Error: {}", e);
            e
        })?;

    #[cfg(feature="twitter_ready")]
    {
        info!("Tweeting.");
        twitter::run(data, format!("{} {}", title, url))
            .map_err(|e| {
                error!("twitter failed to run! Error: {}", e);
                e
            })?;
    }

    Ok(())
}
