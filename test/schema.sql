create table if not exists address
(
	id int auto_increment,
	address varchar(255) null,
	number int null,
	constraint table_name_id_uindex
		unique (id)
);

create table if not exists users
(
	id int auto_increment
		primary key,
	firstname varchar(255) null,
	lastname varchar(255) null,
	random varchar(255) null,
	age int null,
	thing int null,
	address_id int null,
	occured_at datetime null,
	constraint users_address_id_fk
		foreign key (address_id) references address (id)
);