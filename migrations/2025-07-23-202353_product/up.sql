CREATE TABLE product (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    title VARCHAR(50) NOT NULL,
    price FLOAT NOT NULL,
    description VARCHAR(200) NOT NULL,
    slug VARCHAR(50),
    stock INT NOT NULL,
    images TEXT[],
    gender VARCHAR(50)
)
