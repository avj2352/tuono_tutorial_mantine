--# Schema in SQLite to create tables - up.sql
-- User table
CREATE TABLE "users" (
    "_id" INTEGER PRIMARY KEY,
    "name" TEXT NOT NULL,
    "email" TEXT NOT NULL,
    "vendor" TEXT NOT NULL
);

-- Timer table
CREATE TABLE "timers" (
    "_id" INTEGER PRIMARY KEY,
    "title" TEXT NOT NULL,
    "description" TEXT,
    "created_by" TEXT NOT NULL,
    "created_date" TEXT NOT NULL,
    "type" TEXT NOT NULL,
    "time" TEXT NOT NULL,
    FOREIGN KEY ("created_by") REFERENCES "users" ("email")
);

-- Index for faster searches on Timer table
CREATE INDEX idx_timer_title ON "timers" ("title");
CREATE INDEX idx_timer_created_by ON "timers" ("created_by");
CREATE INDEX idx_timer_created_date ON "timers" ("created_date");

---------------------------------------------------------------

--# down.sql - DROP Tables
DROP TABLE [IF EXISTS] "User"
DROP TABLE [IF EXISTS] "Timer"

DROP INDEX [IF EXISTS] idx_timer_title
DROP INDEX [IF EXISTS] idx_timer_created_by
DROP INDEX [IF EXISTS] idx_timer_created_date

---------------------------------------------------------------
