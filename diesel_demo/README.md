## 使用步骤

步骤不能错

0. 这一步可以忽略，因为当前项目已经将db文件上传了

> echo "DATABASE_URL=diesel_demo.db" > .env

1. 生成sql

> diesel migration generate create_posts

2. 生成db文件

> diesel migration run

## Tests

```bash
$ cargo run --bin show_posts

$ cargo run --bin write_post
# 写一些内容，然后Ctrl+D 退出就可以保存数据

$ cargo run --bin publish_post 1

$ cargo run --bin show_posts
# 看一下数据

# 删除一条数据，根据title字段模糊匹配
$ cargo run --bin delete_post "hello"

$ cargo run --bin show_posts
# 再看一下数据
```
