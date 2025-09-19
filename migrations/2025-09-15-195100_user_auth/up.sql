-- Your SQL goes here
CREATE TABLE user_auth (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
  email VARCHAR(30) NOT NULL,
  password VARCHAR(60) NOT NULL,
  fullname VARCHAR(50) NOT NULL,
  is_active boolean,
  roles VARCHAR(20)[]
)
