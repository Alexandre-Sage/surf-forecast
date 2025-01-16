-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
	id uuid primary key unique not null,
	email varchar(100) unique not null,
	user_name varchar(25) unique not null,
	first_name varchar(25) not null, 
	last_name varchar(50) not null,
	password varchar(250) not null,
	created_at timestamp not null
);
