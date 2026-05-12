-- Add down migration script here

DROP TABLE IF EXISTS "users";
DROP TABLE IF EXISTS "user_role";

DROP EXTNSION IF EXISTS "uuid-ossp";
