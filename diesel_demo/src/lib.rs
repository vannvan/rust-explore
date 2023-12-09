pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
// use models::Post;
use std::env;

use self::models::{NewPost, Post};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) {
//     use crate::schema::posts;

//     let new_post = NewPost { title, body };

//     let result = diesel::insert_or_ignore_into(posts::table)
//         .values(&new_post)
//         .execute(conn)
//         .expect("保存失败");

//     println!("写入结果: {:?}", result);
// }

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost {
        title,
        body,
        // id: &32,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

// pub fn publish_post

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert_item() {
        let connection = &mut establish_connection();

        let title = String::from("title2");
        let body = String::from("body2");

        let post = create_post(connection, &title, &body);
        println!("\nSaved draft {title} with id {}", post.id);
    }
}
