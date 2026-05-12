-- PGS = PostgressSQL

-- CREATE TYPE id_role AS ENUM ('admin','user'); 
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; 
-- CREATE TABLE IF NOT EXISTS users (
--   id UUID NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(), //PGS
--
--   user_name VARCHAR(100) NOT NULL,
--   user_mobile VARCHAR(10),
--   user_email VARCHAR(100) UNIQUE,
--   user_dob TIMESTAMP WITH TIME ZONE,
--
--   user_verified BOOLEAN NOT NULL DEFAULT FALSE,
--   user_password VARCHAR(100) NOT NULL,
--
--   user_verification_token VARCHAR(255),
--   user_token_expires_at TIMESTAMP WITH TIME ZONE,
--
--   user_role id_role NOT NULL DEFAULT 'user',
--   user_created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
--   user_updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
-- );
--
-- CREATE INDEX users_email_idx ON users (user_email);



-- SQLite-compatible migration
CREATE TABLE IF NOT EXISTS users (
  id TEXT NOT NULL PRIMARY KEY,

  user_name TEXT NOT NULL,
  user_mobile TEXT CHECK (length(user_mobile)<=13), -- +91 ...
  user_email TEXT NOT NULL UNIQUE,
  user_dob TEXT, -- time format "2026-05-05 20:30:00"


  user_verified INTEGER NOT NULL DEFAULT 0,  -- BOOL => INT 0/1
  user_password TEXT NOT NULL,

  user_verification_token TEXT,
  user_token_expires_at TEXT, -- time format "2026-05-05 20:30:00"

  user_role TEXT NOT NULL DEFAULT 'user' CHECK (user_role IN ('admin', 'user')),

  user_created_at TEXT DEFAULT CURRENT_TIMESTAMP, -- time format "2026-05-05 20:30:00"
  user_updated_at TEXT DEFAULT CURRENT_TIMESTAMP -- time format "2026-05-05 20:30:00"
);

