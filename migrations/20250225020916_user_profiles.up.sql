-- Add up migration script here
START TRANSACTION;

CREATE TABLE IF NOT EXISTS `user_profiles`
(
    `id`                bigint unsigned NOT NULL AUTO_INCREMENT,
    `user_id`           bigint unsigned NOT NULL,
    `nickname`          varchar(50)     NOT NULL DEFAULT '',
    `avatar`            varchar(255)    NOT NULL DEFAULT '',
    `current_status`    varchar(255)    NOT NULL DEFAULT '',
    `bio`               varchar(255)    NOT NULL DEFAULT '',
    `created_at`        timestamp       NULL     DEFAULT NULL,
    `updated_at`        timestamp       NULL     DEFAULT NULL,
    `deleted_at`        timestamp       NULL     DEFAULT NULL,
    PRIMARY KEY (`id`),
    UNIQUE KEY `user_profiles_user_id_unique` (`user_id`),
    INDEX `user_profiles_nickname_index` (`nickname`)
) ENGINE = InnoDB
    DEFAULT CHARSET = utf8mb4
    COLLATE = utf8mb4_unicode_ci;

COMMIT;