## article api

create

```bash
curl -X POST -H "Content-Type: application/json" http://127.0.0.1:8080/v1/articles -d '
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
curl http://127.0.0.1:8080/v1/articles/1
curl http://127.0.0.1:8080/v1/articles/0
curl http://127.0.0.1:8080/v1/articles/-1
curl http://127.0.0.1:8080/v1/articles/s
```

lists

```bash
curl http://127.0.0.1:8080/v1/articles
curl "http://127.0.0.1:8080/v1/articles?page=-1&page_size=s"
```

update

```bash
curl -X PUT -H "Content-Type: application/json" http://127.0.0.1:8080/v1/articles/1 -d '
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
curl -X DELETE http://127.0.0.1:8080/v1/articles/1
```


## seeds

```bash
DELIMITER //

CREATE PROCEDURE InsertArticles(num_articles INT)
BEGIN
    DECLARE i INT DEFAULT 1;
    WHILE i <= num_articles DO
        INSERT INTO `rust_api`.`articles` (category_id, title, author, source, source_url, thumbnail, summary, content, status, created_at, updated_at)
        VALUES (1, CONCAT('title', i), 'admin', CONCAT('source', i), CONCAT('source_url', i), CONCAT('thumbnail', i), CONCAT('summary', i), CONCAT('content', i), 1, NOW(), NOW());
        SET i = i + 1;
    END WHILE;
END //

DELIMITER ;

CALL InsertArticles(50); -- insert 10 rows
```
