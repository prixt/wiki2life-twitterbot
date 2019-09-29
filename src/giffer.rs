use std::fs::File;
use std::path::Path;
use std::error::Error;

use gif::*;

pub struct Giffer<'a> {
    width: u16, height: u16,
    colors: [u8;6],
    frames: Vec<Frame<'a>>,
}

impl<'a> Giffer<'a> {
    pub fn new(width: u16, height: u16, live_color: [u8;3], dead_color: [u8;3]) -> Self
    {
        Self {
            width, height,
            colors: [dead_color[0], dead_color[1], dead_color[2], live_color[0], live_color[1], live_color[2]],
            frames: vec![],
        }
    }

    pub fn add_frame(&mut self, frame_data: &[bool], delay: u16)
    {
        let mut pixels: Vec<u8> = frame_data.iter().map(|b| *b as u8).collect();
        let mut frame = Frame::from_indexed_pixels(
            self.width,
            self.height,
            pixels.as_mut_slice(),
            None,
        );
        frame.delay = delay;
        self.frames.push(frame);
    }

    pub fn write_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>>
    {
        let mut file = File::create(path)?;

        let mut encoder = Encoder::new(
            &mut file,
            self.width, self.height,
            &self.colors
        )?;
        encoder.set(Repeat::Infinite)?;

        for frame in self.frames.iter() {
            encoder.write_frame(frame)?;
        }

        Ok(())
    }
}