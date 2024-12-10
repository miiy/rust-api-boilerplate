-- Add up migration script here
START TRANSACTION;

CREATE TABLE IF NOT EXISTS `users`
(
    `id`                 bigint unsigned NOT NULL AUTO_INCREMENT,
    `name`               varchar(255)    NOT NULL DEFAULT '',
    `password`           varchar(255)    NOT NULL DEFAULT '',
    `email`              varchar(255)    NOT NULL DEFAULT '',
    `created_at`         timestamp       NULL     DEFAULT NULL,
    `updated_at`         timestamp       NULL     DEFAULT NULL,
    `deleted_at`         timestamp       NULL     DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `users_name_unique` (`name`)
    ) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_unicode_ci;


COMMIT;
