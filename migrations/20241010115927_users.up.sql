-- Add up migration script here
START TRANSACTION;

CREATE TABLE IF NOT EXISTS `users`
(
    `id`                 bigint unsigned NOT NULL AUTO_INCREMENT,
    `username`           varchar(255)    NOT NULL DEFAULT '',
    `password`           varchar(255)    NOT NULL DEFAULT '',
    `email`              varchar(255)    NOT NULL DEFAULT '',
    `phone`              varchar(255)    NOT NULL DEFAULT '',
    `status`             tinyint         NOT NULL DEFAULT 0 COMMENT '0: disabled, 1: enabled',
    `created_at`         timestamp       NULL     DEFAULT NULL,
    `updated_at`         timestamp       NULL     DEFAULT NULL,
    `deleted_at`         timestamp       NULL     DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `users_username_unique` (`username`),
    UNIQUE KEY `users_email_unique` (`email`),
    UNIQUE KEY `users_phone_unique` (`phone`)
) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_unicode_ci;


COMMIT;
