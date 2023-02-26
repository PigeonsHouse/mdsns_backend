-- This file should undo anything in `up.sql`
DROP TRIGGER trig_update_post ON "posts";
DROP TABLE "posts";