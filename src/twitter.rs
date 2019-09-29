use std::error::Error;

use tokio::runtime::current_thread::block_on_all;
use egg_mode::media::{UploadBuilder, media_types};
use egg_mode::tweet::DraftTweet;
use egg_mode::{KeyPair, Token::Access};

pub fn run(data: Box<[u8]>, description: &str) -> Result<(), Box<dyn Error>> {
    let consumer = KeyPair::new(env!("CONSUMER_KEY"), env!("CONSUMER_SECRET"));
    let access = KeyPair::new(env!("ACCESS_KEY"), env!("ACCESS_SECRET"));
    let token = Access {consumer,access};

    let _twitter_user = block_on_all(egg_mode::verify_tokens(&token))?;
    
    let builder = UploadBuilder::new(data.as_ref(), media_types::image_gif());
    let media_handle = block_on_all(builder.call(&token))?;

    let draft = DraftTweet::new(description)
        .media_ids(&[media_handle.id]);
    
    let _tweet = block_on_all(draft.send(&token))?;

    Ok(())
}