use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::error::Error;

use tokio::runtime::current_thread::block_on_all;
use egg_mode::media::{UploadBuilder, media_types};
use egg_mode::tweet::DraftTweet;
use egg_mode::{KeyPair, Token::Access};

pub fn run(path: &Path) -> Result<(), Box<dyn Error>> {
    let consumer = KeyPair::new(env!("CONSUMER_KEY"), env!("CONSUMER_SECRET"));
    let access = KeyPair::new(env!("ACCESS_KEY"), env!("ACCESS_SECRET"));
    let token = Access {consumer,access};

    let _twitter_user = block_on_all(egg_mode::verify_tokens(&token))?;
    
    let mut data = vec![];
    File::open(path)?
        .read_to_end(&mut data)?;
    let builder = UploadBuilder::new(&data, media_types::image_gif());
    let media_handle = block_on_all(builder.call(&token))?;

    let draft = DraftTweet::new("Have a gif!")
        .media_ids(&[media_handle.id]);
    
    let _tweet = block_on_all(draft.send(&token))?;

    Ok(())
}