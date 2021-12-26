-- Your SQL goes here
CREATE TABLE job
(
    id                 UUID PRIMARY KEY,
    name               VARCHAR   NOT NULL,
    total_size         INT       NOT NULL,
    downloaded_size    INT       NOT NULL,
    percent_downloaded INT       NOT NULL,
    status             VARCHAR   NOT NULL,
    is_active          BOOLEAN   NOT NULL,
    creation_date      TIMESTAMP NOT NULL,
    expiration_date    TIMESTAMP
);