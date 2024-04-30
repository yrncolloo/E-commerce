drop table if exists status_update cascade;
drop table if exists sale_and_product cascade;
drop table if exists sale cascade;
drop table if exists customer_and_addres cascades;
drop table if exists customer cascade;
drop table if exists address cascade;
drop table if exists product cascade;
drop table if exists subcategory cascade;
drop table if exists category cascade;
drop table if exists product_review cascade;
-- Deallocate all prepared statements
deallocate all;

create table category (
	id int generated always as identity primary key,
	name varchar(50) unique not null,
	logo varchar(50)
);

create table subcategory (
	id int generated always as identity primary key,
	category_id int not null,
	name varchar(50) unique not null,
	foreign key (category_id) references category (id)
);

create table product (
	id int generated always as identity primary key,
	name varchar(100) not null,
	description varchar(5000) not null,
	image_id varchar(36),
	price numeric(20, 4) not null,
	stock int not null,
	subcategory_id int not null,
	foreign key (subcategory_id) references subcategory (id)
);

create table address (
	id int generated always as identity primary key,
	type varchar(50) not null,
	country varchar(50) not null,
	state varchar(50) not null,
	town varchar(50) not null,
	zip varchar(20) not null,
	address_line_1 varchar(100) not null,
	address_line_2 varchar(100)
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
	token text default null,
	foreign key (default_address_id) references address (id)
);

create table customer_and_address (
	id int generated always as identity primary key,
	customer_id int not null,
	address_id int not null,
	foreign key (customer_id) references customer (id),
	foreign key (address_id) references address (id)
);

create table sale (
	id int generated always as identity primary key,
	ordered_at timestamp without time zone not null,
	shipping_status varchar(30) default 'Not sent' not null,
	customer_id int not null,
	address_id int not null,
	is_paid boolean default false not null,
	foreign key (customer_id) references customer (id),
	foreign key (address_id) references address (id)
);

create table sale_and_product (
	id int generated always as identity primary key,
	sale_id int not null,
	product_id int not null,
	amount int not null,
	price_per_piece decimal(20, 4) not null,
	foreign key (sale_id) references sale (id),
	foreign key (product_id) references product (id)
);

create table status_update (
	id int generated always as identity primary key,
	sale_id int not null,
	status_change varchar(500) not null,
	foreign key (sale_id) references sale (id)
);

create table product_review (
	id int generated always as identity primary key,
	customer_id int not null,
	product_id int not null,
	rating int not null,
	review_text varchar(1000),
	review_date timestamp without time zone default current_timestamp,
	foreign key (customer_id) references customer (id),
	foreign key (product_id) references product (id)
);

