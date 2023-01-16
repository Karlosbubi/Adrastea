use std::error::Error;

use crate::settings::{self, DB};
use diesel::Queryable;
use rusqlite::{params, Connection, Result};

#[derive(Debug, Clone, Queryable)]
pub struct Feed {
    pub id: i32,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Queryable)]
pub struct Article {
    pub title: String,
    pub version: i32,
    pub path: String,
    pub feed: i32,
    pub url: String,
}

pub struct DbAdapter {
    pub db: DB,
    pub location: String,
    pub auth: String,
}

impl DbAdapter {
    pub fn write_article(&self, article: Article) -> Result<(), Box<dyn Error>> {
        match self.db {
            DB::SQLiteBundled => {
                let path = self.location.clone() + "/index.db";
                let conn = Connection::open(path.as_str())?;
                // TODO : Check if table already exists
                _ = conn.execute(
                    "CREATE TABLE article (
                    title TEXT PRIMARY KEY,
                    version INTEGER NOT NULL,
                    path TEXT NOT NULL,
                    feed INTEGER NOT NULL,
                    url TEXT NOT NULL
                )",
                    (),
                );

                _ = conn.execute(
                    "INSERT INTO article (title, version, path, feed, url)
                    VALUES (?1, ?2, ?3, ?4, ?5)",
                    (
                        article.title,
                        article.version,
                        article.path,
                        article.feed,
                        article.url,
                    ),
                );
            }
            DB::SQLiteSystem => todo!(),
            DB::Postgress => todo!(),
            DB::Maria => todo!(),
            DB::MongoDB => todo!(),
        };
        Ok(())
    }

    pub fn write_blob_article(&self, _article: Article, _blob: String) {
        match self.db {
            DB::SQLiteBundled => todo!(),
            DB::SQLiteSystem => todo!(),
            DB::Postgress => todo!(),
            DB::Maria => todo!(),
            DB::MongoDB => todo!(),
        }
    }
}
