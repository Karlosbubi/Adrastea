
use rss::{Channel, Item};

use  crate::{settings::Settings, data::Article};

use std::{thread, time::Duration, error::Error};

pub struct Service<'a> {
    settings : Settings<'a>,
}

impl Service<'_> {
    async fn daemon(&self) -> Result<(), Box<dyn Error>>{
        loop{
            for f in &self.settings.feeds {
                let channel : Channel = Self::get_channel(f.url).await?;
                let posts = channel.items().iter().take(self.settings.check_deep);
                
                for p in posts {
                    Self::process_post(f.id, p);
                }

            }
            thread::sleep(Duration::from_secs(self.settings.check_every * 60));
        }
       // Ok(())
    }

    async fn get_channel(url : &str) -> Result<Channel, Box<dyn Error>> {
        let content = reqwest::get(url)
            .await?
            .bytes()
            .await?;
        let channel = Channel::read_from(&content[..])?;
        Ok(channel)
    }

    fn process_post(feed : i32, post : &Item) -> Result<(), Box<dyn Error>>{
        let url = post.link().unwrap();
        Self::download(url);
        let path = ""; // TODO
        let a : Article = Article { title: post.title().unwrap(), version: 1, path: path, feed: feed, url: url };
        Ok(())
    }

    fn download(url : &str) -> Result<(), Box<dyn Error>>{
        Ok(())
    }
}