-- Add up migration script here


CREATE TYPE user_role as ENUM ('admin','user');
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users"(
  id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_V4()),
  name VARCHAR(100) NOT NULL,
  email VARCHAR(255) NOT NULL UNIQUE,
  verified BOOLEAN NOT NULL DEFAULT FALSE,
  password VARCHAR(100) NOT NULL,
  verification_token VARCHAR(255),
  token_expire_at TIMESTAMP WITH TIME ZONE,
  role user_role NOT NULL DEFAULT 'user',
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX user_email_idx ON users (email);
