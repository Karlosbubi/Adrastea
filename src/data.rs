use diesel::Queryable;

#[derive(Debug, Clone, Queryable)]
pub struct Feed<'a> {
    pub id: i32,
    pub name: &'a str,
    pub url: &'a str,
    pub(crate) link: &str,
}

#[derive(Debug, Clone, Queryable)]
pub struct Article<'a> {
    pub title: &'a str,
    pub version: i32,
    pub path: &'a str,
    pub feed: i32,
    pub url: &'a str,
}