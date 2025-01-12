-- Active: 1736685035407@@127.0.0.1@3306@zerotoprod
-- Add up migration script here
CREATE TABLE IF NOT EXISTS `subscription` (
    `id` BIGINT UNSIGNED PRIMARY KEY,
    `email` VARCHAR(255) NOT NULL UNIQUE,
    `name` VARCHAR(255) NOT NULL,
    `subscribed_at` TIMESTAMP NOT NULL
);