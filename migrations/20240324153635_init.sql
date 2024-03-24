CREATE TABLE "scope" (
    id serial NOT NULL PRIMARY KEY,
    name varchar(32) NOT NULL,
    UNIQUE(name)
);

CREATE TYPE "state" AS ENUM (
    'created',
    'open',
    'acceptes',
    'rejected',
    'archived',
    'redacted',
    'deleted'
);

CREATE TABLE "vote" (
    uuid uuid NOT NULL PRIMARY KEY,
    scope_id serial NOT NULL REFERENCES "scope" (id),
    short_id varchar(16) DEFAULT(NULL), -- id of an accepted vote e.g. B123
    name text NOT NULL,
    text text NOT NULL,
    result json NOT NULL, -- voting options, type and result in one json
    state state DEFAULT('created') NOT NULL,
    t_open timestamp DEFAULT(NULL), -- possible to move these to a "log" table
    t_close timestamp DEFAULT(NULL),
    t_archive timestamp DEFAULT(NULL),
    t_redact timestamp DEFAULT(NULL),
    creater varchar(64) NOT NULL,
    UNIQUE(scope_id, short_id)
);