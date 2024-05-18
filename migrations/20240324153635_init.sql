CREATE TABLE "scopes" (
    id serial NOT NULL PRIMARY KEY,
    name varchar(32) NOT NULL,
    UNIQUE(name)
);

CREATE TYPE "state" AS ENUM (
    'created',
    'open',
    'accepted',
    'rejected',
    'archived',
    'redacted',
    'deleted'
);

CREATE TABLE "tags" (
    id serial NOT NULL PRIMARY KEY,
    tag varchar(64) NOT NULL, -- can fit "Rindfleischetikettierungsüberwachungsaufgabenübertragungsgesetz"
    UNIQUE(tag)
);

CREATE TABLE "votes" (
    id serial NOT NULL PRIMARY KEY,
    uuid uuid NOT NULL,
    scope_id serial NOT NULL REFERENCES "scopes" (id),
    short_id varchar(16) DEFAULT(NULL), -- id of an accepted vote e.g. B123
    name text NOT NULL,
    vote json NOT NULL, -- voting text, options, type and result in one json
    state state DEFAULT('created') NOT NULL,
    t_reminder timestamp DEFAULT(NULL),
    t_open timestamp DEFAULT(NULL),
    t_close timestamp DEFAULT(NULL),
    t_archive timestamp DEFAULT(NULL),
    t_redact timestamp DEFAULT(NULL),
    creator varchar(64) NOT NULL, -- user table todo
    UNIQUE(scope_id, short_id),
    UNIQUE(uuid)
);

CREATE TABLE "tag_assignment" (
    tag_id int NOT NULL REFERENCES "tags" (id),
    vote_id int NOT NULL REFERENCES "votes" (id),
    PRIMARY KEY (tag_id, vote_id)
);

CREATE TABLE "audit_log" (
    id serial NOT NULL PRIMARY KEY,
    vote_id int NOT NULL REFERENCES "votes" (id),
    prev_hash bytea NOT NULL,
    timestamp timestamp DEFAULT(NOW()),
    stuff json NOT NULL, -- todo
    stuff2 json NOT NULL -- todo 2, deletable stuff
);