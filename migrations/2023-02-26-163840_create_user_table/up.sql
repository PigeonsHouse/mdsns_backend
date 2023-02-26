-- Your SQL goes here
CREATE TABLE "users" (
                         "id" varchar PRIMARY KEY,
                         "name" varchar(255) NOT NULL,
                         "email" varchar(320) UNIQUE NOT NULL,
                         "description" varchar,
                         "created_at" timestamp NOT NULL DEFAULT now(),
                         "updated_at" timestamp NOT NULL DEFAULT now()
);
CREATE FUNCTION set_update_time() RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'UPDATE') THEN
        NEW.updated_at := now();
return NEW;
END IF;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trig_update_user BEFORE UPDATE ON "users" FOR EACH ROW EXECUTE PROCEDURE set_update_time();
