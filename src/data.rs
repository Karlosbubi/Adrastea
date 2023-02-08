use std::error::Error;

use crate::schema::articles;
use crate::settings::{self, DB};
use diesel::{Connection, Insertable, Queryable, RunQueryDsl};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rusqlite::{params, Result};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Debug, Clone, Queryable)]
pub struct Feed {
    pub id: i32,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = articles)]
pub struct Article {
    pub hash: String,
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
    pub fn write_article(
        &self,
        article: Article,
    ) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        match self.db {
            DB::SQLite => {
                let path = self.location.clone() + "/index.db";
                let mut connection = diesel::SqliteConnection::establish(&path)?;

                connection.run_pending_migrations(MIGRATIONS)?;

                diesel::insert_into(articles::table)
                    .values(&article)
                    .execute(&mut connection)
                    .expect("Error saving new post");
            }
            DB::Postgress => {
                todo!(); // TODO connection string
                let path = self.location + &self.auth;
                let mut connection = diesel::PgConnection::establish(&path)?;

                connection.run_pending_migrations(MIGRATIONS)?;

                diesel::insert_into(articles::table)
                    .values(&article)
                    .execute(&mut connection)
                    .expect("Error saving new post");
            }
            DB::Maria => todo!(),
            DB::MongoDB => todo!(),
            DB::SurrealDB => todo!(),
        };
        Ok(())
    }

    pub fn write_blob_article(&self, _article: Article, _blob: String) {
        match self.db {
            DB::SQLite => todo!(),
            DB::Postgress => todo!(),
            DB::Maria => todo!(),
            DB::MongoDB => todo!(),
            DB::SurrealDB => todo!(),
        }
    }

    pub fn write_feed(&self, _feed: Feed) {
        // TODO
    }

    pub fn read_feeds(&self) -> Vec<Feed> {
        todo!(); // TODO
    }
}
