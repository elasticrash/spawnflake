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

