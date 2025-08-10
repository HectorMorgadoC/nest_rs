-- Your SQL goes here
CREATE TABLE product_image (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    url VARCHAR(200) NOT NULL,
    product_id UUID NOT NULL,
    FOREIGN KEY (product_id) REFERENCES product(id)
)
