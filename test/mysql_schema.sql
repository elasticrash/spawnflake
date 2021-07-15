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

create table numeric_types
(
	id int auto_increment
		primary key,
	int_type int null,
	smallint_type smallint null,
	tinint_type tinyint null,
	mediumint_type mediumint null,
	bigint_type bigint null,
	decimal_type decimal null,
	demical_2_type decimal(10,5) null,
	float_type float null,
	double_type double null,
	bit_type bit null,
	unsigned_int_type int unsigned null,
	unsigned_tinyint_type tinyint unsigned null,
	unsigned_bigint_type bigint unsigned null
);

create table if not exists date_types
(
	id int auto_increment
		primary key,
	date_type date null,
	datetime_type datetime null,
	timestamp_type timestamp null,
	time_type time null,
	year_type year null
);

create table if not exists string_types
(
	id int auto_increment
		primary key,
	char_type char(2) null,
	binary_type binary(5) null,
	text_type text null,
	blob_type blob null,
	varchar_type varchar(44) null
);

create table if not exists no_auto_increment
(
	id bigint not null
		primary key,
	text longtext null,
	address_id int null,
	constraint table_name_address_id_fk
		foreign key (address_id) references address (id)
);

