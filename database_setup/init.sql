CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE product (
    id SERIAL PRIMARY KEY,
    product_name VARCHAR(255) NOT NULL UNIQUE,
    price DECIMAL(10, 2) NOT NULL,
    star INTEGER CHECK (star >= 0 AND star <= 5),
    image_name VARCHAR(255) NOT NULL,
    date_added TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    category_id INTEGER NOT NULL,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity >= 0),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

CREATE TABLE colors (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    color VARCHAR(50) NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity >= 0),
    FOREIGN KEY (product_id) REFERENCES product(id) ON DELETE CASCADE
);

CREATE TABLE sizes (
    id SERIAL PRIMARY KEY,
    product_id INTEGER NOT NULL,
    size VARCHAR(50) NOT NULL,
    quantity INTEGER NOT NULL CHECK (quantity >= 0),
    FOREIGN KEY (product_id) REFERENCES product(id) ON DELETE CASCADE
);

create table customer (
	id int generated always as identity primary key,
	username varchar(50) not null unique,
	first_name varchar(50) not null,
	last_name varchar(50) not null,
	email varchar(320) not null,
	telephone varchar(320) not null,
	default_address_id int unique,
	salt varchar(64) unique not null,
	password_hash varchar(64) not null,
	token text default null
);

create table address (
	id int generated always as identity primary key,
	country varchar(50) not null,
	state varchar(50) not null,
	town varchar(50) not null,
	zip varchar(20) not null,
	address_line_1 varchar(100) not null,
	address_line_2 varchar(100)
);


create table product_review (
	id int generated always as identity primary key,
	customer_id int not null,
	product_id int not null,
	rating int not null,
	review_text varchar(1000) not null,
	review_date timestamp without time zone default current_timestamp not null
);
