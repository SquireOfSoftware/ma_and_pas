-- auto binding new db creations
CREATE DATABASE orders;

CREATE TABLE IF NOT EXISTS people (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    created_date TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE IF NOT EXISTS orders (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    created_date TIMESTAMP WITH TIME ZONE NOT NULL,
    submitted_date TIMESTAMP WITH TIME ZONE, -- when the order was made
    delivered_date TIMESTAMP WITH TIME ZONE, -- when the order was delivered
    cost serial, -- null means its free, measures cents
    person_id uuid,
    CONSTRAINT fk_person FOREIGN KEY(person_id) REFERENCES people(id)
);

CREATE INDEX idx_created_date ON orders (created_date);
CREATE INDEX idx_submitted_date ON orders (submitted_date);
CREATE INDEX idx_delivered_date ON orders (delivered_date);

CREATE TABLE IF NOT EXISTS burgers (
    code_name VARCHAR UNIQUE PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    cost serial,
    type VARCHAR,
    active boolean NOT NULL DEFAULT TRUE
);

CREATE INDEX idx_burger_type ON burgers(type);

CREATE TABLE IF NOT EXISTS order_burgers (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    burger_id VARCHAR,
    order_id uuid,
    CONSTRAINT fk_order FOREIGN KEY(order_id) REFERENCES orders(id),
    CONSTRAINT fk_burger FOREIGN KEY(burger_id) REFERENCES burgers(code_name)
);

CREATE TABLE IF NOT EXISTS sides (
    code_name VARCHAR UNIQUE NOT NULL,
    name VARCHAR NOT NULL,
    cost serial,
    size VARCHAR,
    type VARCHAR,
    active boolean NOT NULL DEFAULT TRUE,
    PRIMARY KEY(code_name, size, type)
);

CREATE INDEX idx_side_type ON sides(type);

CREATE TABLE IF NOT EXISTS order_sides (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    side_id VARCHAR,
    side_size VARCHAR,
    side_type VARCHAR,
    order_id uuid,
    CONSTRAINT fk_order FOREIGN KEY(order_id) REFERENCES orders(id),
    CONSTRAINT fk_side FOREIGN KEY(side_id, side_size, side_type) REFERENCES sides(code_name, size, type)
);

INSERT INTO burgers (code_name, name, active, type, cost)
VALUES ('cheese_burger', 'Cheese Burger', True, 'cheese', 200);

INSERT INTO sides (code_name, name, active, type, size, cost)
VALUES ('small_coke', 'Small Coke', True, 'drink', 'small', 150),
       ('medium_coke', 'Medium Coke', True, 'drink', 'medium', 300),
       ('small_fries', 'Small Fries', True, 'fries', 'small', 200),
       ('large_fries', 'Large Fries', True, 'fries', 'large', 350);

CREATE TABLE IF NOT EXISTS meals (
    code_name VARCHAR UNIQUE PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    cost serial,
    active boolean NOT NULL DEFAULT TRUE
);

CREATE INDEX idx_meals_active ON meals(active);

INSERT INTO meals (code_name, name, active, cost)
VALUES ('happy_meal', 'Happy Meal', TRUE, 400);

CREATE TABLE IF NOT EXISTS order_meals (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    meal_id VARCHAR,
    order_id uuid,
    CONSTRAINT fk_order FOREIGN KEY(order_id) REFERENCES orders(id),
    CONSTRAINT fk_meal FOREIGN KEY(meal_id) REFERENCES meals(code_name)
);

CREATE TABLE IF NOT EXISTS meals_burgers (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    burger_id VARCHAR,
    meal_id VARCHAR,
    CONSTRAINT fk_meal FOREIGN KEY(meal_id) REFERENCES meals(code_name),
    CONSTRAINT fk_burger FOREIGN KEY(burger_id) REFERENCES burgers(code_name)
);

CREATE TABLE IF NOT EXISTS meals_sides (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    side_id VARCHAR,
    side_size VARCHAR,
    side_type VARCHAR,
    meal_id VARCHAR,
    CONSTRAINT fk_meal FOREIGN KEY(meal_id) REFERENCES meals(code_name),
    CONSTRAINT fk_side FOREIGN KEY(side_id, side_size, side_type) REFERENCES sides(code_name, size, type)
);

INSERT INTO people (first_name, last_name, created_date) values ('john', 'smith', now());