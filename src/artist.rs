#[derive(Deserialize, Debug)]
pub struct Alias {
    pub name: String,
    pub sort_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Date {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub date: Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub count: i64,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct Rating {
    pub count: i64,
    pub value: i64,
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub id: i64,
    pub gid: String,
    pub name: String,
    pub sort_name: String,
    pub area: Option<String>,
    pub aliases: Option<Vec<Alias>>,
    pub begin: Option<Date>,
    pub end: Option<Date>,
    pub tags: Option<Vec<Tag>>,
    pub rating: Option<Rating>,
}
