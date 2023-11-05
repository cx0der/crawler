CREATE USER crawler WITH PASSWORD 'crawler';
CREATE DATABASE  IF NOT EXISTS crawler;
GRANT ALL PRIVILEGES ON DATABASE crawler TO crawler;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


CREATE TABLE IF NOT EXISTS feed
(
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    url TEXT NOT NULL,
    icon_url TEXT NOT NULL,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS article
(
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    feed_id UUID NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    url TEXT NOT NULL,
    published_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (id)
);
