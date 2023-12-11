use diesel::prelude::*;
use diesel_demo::models::*;
use diesel_demo::*;

fn main() {
    use diesel_demo::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        // .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("title: {},body: {}", post.title, post.body);
        println!("----------\n");
        // println!("{}", post.body);
    }
}
