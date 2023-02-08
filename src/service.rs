use rss::{Channel, Item};
use sha2::{Digest, Sha256};
use std::io;

use crate::{
    data::{Article, DbAdapter},
    settings::Settings,
};

use reqwest;

use std::{error::Error, fs, thread, time::Duration};

pub struct Service {
    pub(crate) settings: Settings,
    db: DbAdapter,
}

impl Service {
    pub fn new(settings: Settings) -> Self {
        let db = DbAdapter {
            db: settings.db,
            location: settings.save_root.clone(),
            auth: "".to_string(),
        };
        Self { settings, db }
    }

    pub async fn daemon(&self) -> Result<(), Box<dyn Error>> {
        loop {
            for f in &self.settings.feeds {
                let channel: Channel = Self::get_channel(f.url.as_str()).await?;
                let posts = channel.items().iter().take(self.settings.check_deep);

                for p in posts {
                    Self::process_post(self, f.id, p).await?;
                }
            }
            thread::sleep(Duration::from_secs(self.settings.check_every * 60));
        }
        // Ok(())
    }

    async fn get_channel(url: &str) -> Result<Channel, Box<dyn Error>> {
        let content = reqwest::get(url).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;
        Ok(channel)
    }

    async fn process_post(&self, feed: i32, post: &Item) -> Result<(), Box<dyn Error>> {
        let link = post.link().unwrap().to_string();
        let title = post.title().unwrap().to_string();
        let path = format!("{}/{}/{}.html", self.settings.save_root, feed, title.trim()); // TODO
        let mut a: Article = Article {
            title: title,
            version: 1,
            path: path.clone(),
            feed: feed,
            url: link.clone(),
            hash: "".to_string(),
        };

        let mut hasher = Sha256::new();

        if !self.settings.raw_to_db {
            Self::download(a.url.clone(), path.clone()).await?;
            let mut file = fs::File::open(&path)?;
            _ = io::copy(&mut file, &mut hasher)?;
            let hash = hasher.finalize();
            a.hash = format!("{:x}", hash);
            _ = self.db.write_article(a);
        } else {
            //TODO : Add Whole article to DB
            let resp = reqwest::get(a.url.clone()).await?.text().await?;
            hasher.update(resp);
            let hash = hasher.finalize();
            a.hash = format!("{:x}", hash);
        }

        // DEBUG
        //println!("{}", post.title().unwrap());

        Ok(())
    }

    async fn download(url: String, path: String) -> Result<(), Box<dyn Error>> {
        let resp = reqwest::get(url).await?.text().await?;
        fs::write(path, resp).expect("Unable to write file");
        Ok(())
    }
}
