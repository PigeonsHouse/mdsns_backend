-- This file should undo anything in `up.sql`
DROP TRIGGER trig_update_user ON "users";
DROP FUNCTION set_update_time();
DROP TABLE "users";
