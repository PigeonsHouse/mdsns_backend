-- Your SQL goes here
CREATE TABLE "favorites" (
                             "user_id" varchar,
                             "post_id" Uuid,
                             "created_at" timestamp NOT NULL DEFAULT now(),
                             PRIMARY KEY ("user_id", "post_id")
);

ALTER TABLE "favorites" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");
ALTER TABLE "favorites" ADD FOREIGN KEY ("post_id") REFERENCES "posts" ("id");
