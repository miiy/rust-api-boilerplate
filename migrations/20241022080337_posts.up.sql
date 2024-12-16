-- Add up migration script here
START TRANSACTION;

CREATE TABLE IF NOT EXISTS `posts`
(
    `id`                 bigint unsigned NOT NULL AUTO_INCREMENT,
    `user_id`            bigint unsigned NOT NULL DEFAULT 0,
    `category_id`        bigint unsigned NOT NULL DEFAULT 0,
    `title`              varchar(255)    NOT NULL DEFAULT '',
    `author`             varchar(255)    NOT NULL DEFAULT '',
    `source`             varchar(255)    NOT NULL DEFAULT '',
    `source_url`         varchar(255)    NOT NULL DEFAULT '',
    `thumbnail`          varchar(255)    NOT NULL DEFAULT '',
    `summary`            varchar(255)    NOT NULL DEFAULT '',
    `content`            text                     DEFAULT NULL,
    `status`             tinyint         NOT NULL DEFAULT 0 COMMENT '0: draft, 1: published, 2: disabled',
    `created_at`         timestamp       NULL     DEFAULT NULL,
    `updated_at`         timestamp       NULL     DEFAULT NULL,
    `deleted_at`         timestamp       NULL     DEFAULT NULL,
    PRIMARY KEY (`id`),
    INDEX `posts_user_id_index` (`user_id`),
    INDEX `posts_category_id_index` (`category_id`),
    INDEX `posts_created_at_index` (`created_at`)
) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_unicode_ci;


COMMIT;
