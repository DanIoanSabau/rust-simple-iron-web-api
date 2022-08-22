use chrono::DateTime;
use chrono::offset::Utc;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    title: String,
    body: String,
    author: String,
    datetime: DateTime<Utc>,
    uuid: Uuid,
}


impl Post {

    pub fn new(
        title: &str,
        body: &str,
        author: &str,
        datetime: DateTime<Utc>,
        uuid: Uuid,
    ) -> Post {
        return Post {
            title: title.to_string(),
            body: body.to_string(),
            author: author.to_string(),
            datetime,
            uuid,
        };
    }

    pub fn uuid(&self) -> &Uuid {
        return &self.uuid
    }
}