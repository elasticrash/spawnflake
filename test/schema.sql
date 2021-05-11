create table if not exists users
(
	id int not null primary key,
	firstname varchar(255) null,
	lastname varchar(255) null,
	age int null
);