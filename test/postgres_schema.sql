create table address
(
	id serial not null
		constraint address_pk
			primary key,
	address varchar,
	number integer
);

alter table address owner to postgres;

create table users
(
	id serial not null
		constraint users_pk
			primary key,
	firstname varchar,
	lastname varchar,
	random varchar,
	age integer,
	thing integer,
	address_id integer
		constraint users_address_id_fk
			references address,
	occured_at timestamp
);

alter table users owner to postgres;

create table numeric_types
(
	id serial not null
		constraint numeric_types_pk
			primary key,
	smallint_type smallint,
	bigint_types bigint,
	decimal_type numeric,
	real_type real,
	double_type double precision
);

alter table numeric_types owner to postgres;

create table date_types
(
	id serial not null
		constraint table_name_pk
			primary key,
	timestamp_type timestamp,
	date_type date,
	time_type time,
	interval_type interval
);

alter table date_types owner to postgres;

create table string_types
(
	id serial not null
		constraint string_types_pk
			primary key,
	varchar_type varchar,
	char_type char,
	text_type text
);

alter table string_types owner to postgres;

create table single_types
(
	id serial not null
		constraint single_types_pk
			primary key,
	byte_type bytea,
	boolean_type boolean
);

alter table single_types owner to postgres;

