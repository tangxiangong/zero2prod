-- Active: 1736685035407@@127.0.0.1@3306@zerotoprod
-- Add up migration script here
CREATE TABLE IF NOT EXISTS `subscription` (
    `id` BIGINT UNSIGNED PRIMARY KEY,
    `email` VARCHAR(50) NOT NULL UNIQUE,
    `name` VARCHAR(50) NOT NULL,
    `subscribed_at` DATETIME NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;