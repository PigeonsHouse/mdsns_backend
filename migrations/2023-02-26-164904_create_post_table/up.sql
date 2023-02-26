-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE "posts" (
                         "id" Uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
                         "author_id" varchar NOT NULL,
                         "content_md" varchar NOT NULL,
                         "content_html" varchar NOT NULL,
                         "reply_at" Uuid,
                         "created_at" timestamp NOT NULL DEFAULT now(),
                         "updated_at" timestamp NOT NULL DEFAULT now()
);

ALTER TABLE "posts" ADD FOREIGN KEY ("author_id") REFERENCES "users" ("id");
ALTER TABLE "posts" ADD FOREIGN KEY ("reply_at") REFERENCES "posts" ("id");

CREATE TRIGGER trig_update_post BEFORE UPDATE ON "posts" FOR EACH ROW EXECUTE PROCEDURE set_update_time();
