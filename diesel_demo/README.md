```bash
$ cargo run --bin show_posts

$ cargo run --bin write_post
# write your post

$ cargo run --bin publish_post 1

$ cargo run --bin show_posts
# your post will be printed here

# Delete post with given title
$ cargo run --bin delete_post "title of post to delete"

$ cargo run --bin show_posts
# observe that no posts are shown
```
