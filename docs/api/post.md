## post api

create

```bash
curl -X POST -H "Content-Type: application/json" http://127.0.0.1:8080/v1/posts -d '
{
    "category_id": 1,
    "title": "title",
    "author": "admin",
    "source": "source",
    "source_url": "source_url",
    "thumbnail": "thumbnail",
    "summary": "summary",
    "content": "content"
}
'
```

detail

```bash
curl http://127.0.0.1:8080/v1/posts/1
curl http://127.0.0.1:8080/v1/posts/0
curl http://127.0.0.1:8080/v1/posts/-1
curl http://127.0.0.1:8080/v1/posts/s
```

lists

```bash
curl http://127.0.0.1:8080/v1/posts
curl "http://127.0.0.1:8080/v1/posts?page=-1&page_size=s"
```

update

```bash
curl -X PUT -H "Content-Type: application/json" http://127.0.0.1:8080/v1/posts/1 -d '
{
    "category_id": 2,
    "title": "title1",
    "author": "admin1",
    "source": "source1",
    "source_url": "source_url1",
    "thumbnail": "thumbnail1",
    "summary": "summary1",
    "content": "content1"
}
'
```

delete

```bash
curl -X DELETE http://127.0.0.1:8080/v1/posts/1
```


## seeds

```bash
DELIMITER //

CREATE PROCEDURE InsertPosts(num_posts INT)
BEGIN
    DECLARE i INT DEFAULT 1;
    WHILE i <= num_posts DO
        INSERT INTO `rust_api`.`posts` (category_id, title, author, content, created_at, updated_at)
        VALUES (1, CONCAT('title', i), CONCAT('admin', i), CONCAT('content', i), NOW(), NOW());
        SET i = i + 1;
    END WHILE;
END //

DELIMITER ;

CALL InsertPosts(50); -- insert 10 rows
```
