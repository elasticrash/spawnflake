create table if not exists users
(
	id int auto_increment primary key,
	firstname varchar(255) null,
	lastname varchar(255) null,
	random varchar(255) null,
	age int null,
	thing int null
);