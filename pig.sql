begin;
set client_min_messages to error;
drop schema if exists pig cascade;
create schema pig;

----------------------------
--------------------- TABLES
----------------------------

create table pig.people (
	id serial primary key,
	name text not null unique constraint person_name check (length(name) > 0)
);

create table pig.things (
	id serial primary key,
	person_id integer not null references pig.people(id),
	name text not null constraint thing_name check (length(name) > 0),
	unique(person_id, name)
);

create table pig.apikeys (
	apikey char(4) primary key,
	person_id integer not null references pig.people(id)
);


----------------------------
------------------ TEST DATA
-- 1 = Ada, API key 'aaaa', has chair and dress
-- 2 = Bai, API key 'bbbb', has egg and fish
----------------------------

insert into pig.people(name) values ('Ada'), ('Bai');
insert into pig.things(person_id, name) values (1, 'chair'), (1, 'dress'), (2, 'egg'), (2, 'fish');
insert into pig.apikeys(apikey, person_id) values ('aaaa', 1), ('bbbb', 2);


----------------------------
-------------- API FUNCTIONS
---- each returns status, js
----------------------------
--- assumes that after auth,
-- API server passes user id
-- where needed, so user can
-- only see/edit own things
----------------------------

-- Auth before every API call
-- IN : 4-char API key
-- OUT: person_id || {}
create function pig.apikey_get(char(4),
	out status smallint, out js json) as $$
begin
	status := 200;
	js := row_to_json(r) from (
		select person_id
		from pig.apikeys
		where apikey =  $1
	) r;
	if js is null then
		status := 404;
		js := '{}';
	end if;
end;
$$ language plpgsql;

-- List all people
-- IN : (nothing)
-- OUT: id, name of all people || []
create function pig.people_get(
	out status smallint, out js json) as $$
begin
	status := 200;
	js := coalesce((
		select json_agg(r) from (
			select id, name
			from pig.people
			order by id
		) r
	), '[]');
end;
$$ language plpgsql;

-- Get one person
-- IN : people.id
-- OUT: id, name of that person || {}
create function pig.person_get(integer,
	out status smallint, out js json) as $$
begin
	status := 200;
	js := row_to_json(r) from (
		select id, name
		from pig.people
		where id =  $1
	) r;
	if js is null then
		status := 404;
		js := '{}';
	end if;
end;
$$ language plpgsql;

-- Update person's name
-- IN : people.id, new name
-- OUT: id, new name of that person || {} if not found || error if bad name
create function pig.person_update(integer, text,
	out status smallint, out js json) as $$
declare
	e6 text; e7 text; e8 text; e9 text;
begin
	update pig.people
	set name = $2
	where id = $1;
	select x.status, x.js into status, js from pig.person_get($1) x;
exception
	when others then get stacked diagnostics e6=returned_sqlstate, e7=message_text, e8=pg_exception_detail, e9=pg_exception_context;
	js := json_build_object('code',e6,'message',e7,'detail',e8,'context',e9);
	status := 500;
end;
$$ language plpgsql;

-- Things for this person
-- IN : person_id
-- OUT: id, name of things for that person || []
create function pig.things_get(integer,
	out status smallint, out js json) as $$
begin
	status := 200;
	js := coalesce((
		select json_agg(r) from (
			select id, name
			from pig.things
			where person_id = $1
			order by id
		) r
	), '[]');
end;
$$ language plpgsql;

-- Get thing, if both person and thing ID match
-- IN : person_id, thing_id
-- OUT: id, name of thing || {}
create function pig.thing_get(integer, integer,
	out status smallint, out js json) as $$
begin
	status := 200;
	js := row_to_json(r) from (
		select id, name
		from pig.things
		where person_id =  $1
		and id = $2
	) r;
	if js is null then
		status := 404;
		js := '{}';
	end if;
end;
$$ language plpgsql;

-- Update thing name
-- IN : person_id, thing_id, new name
-- OUT: id, new name of thing || {} if not found || error if bad name
create function pig.thing_update(integer, integer, text,
	out status smallint, out js json) as $$
declare
	e6 text; e7 text; e8 text; e9 text;
begin
	update pig.things
	set name = $3
	where id = $2
	and person_id = $1;
	select x.status, x.js into status, js from pig.thing_get($1, $2) x;
exception
	when others then get stacked diagnostics e6=returned_sqlstate, e7=message_text, e8=pg_exception_detail, e9=pg_exception_context;
	js := json_build_object('code',e6,'message',e7,'detail',e8,'context',e9);
	status := 500;
end;
$$ language plpgsql;

-- Add a new thing for this person
-- IN : person_id, new thing name
-- OUT: new id, name of thing || error if bad name or person_id
create function pig.thing_add(integer, text,
	out status smallint, out js json) as $$
declare
	thing_id integer;
	e6 text; e7 text; e8 text; e9 text;
begin
	insert into pig.things (person_id, name)
	values ($1, $2)
	returning id into thing_id;
	select x.status, x.js into status, js from pig.thing_get($1, thing_id) x;
exception
	when others then get stacked diagnostics e6=returned_sqlstate, e7=message_text, e8=pg_exception_detail, e9=pg_exception_context;
	js := json_build_object('code',e6,'message',e7,'detail',e8,'context',e9);
	status := 500;
end;
$$ language plpgsql;

-- Delete a thing (if person matches)
-- IN : person_id, thing_id
-- OUT: old id, name of thing || {} if not found || error
create function pig.thing_delete(integer, integer,
	out status smallint, out js json) as $$
declare
	e6 text; e7 text; e8 text; e9 text;
begin
	select x.status, x.js into status, js from pig.thing_get($1, $2) x;
	delete from pig.things
	where id = $2
	and person_id = $1;
exception
	when others then get stacked diagnostics e6=returned_sqlstate, e7=message_text, e8=pg_exception_detail, e9=pg_exception_context;
	js := json_build_object('code',e6,'message',e7,'detail',e8,'context',e9);
	status := 500;
end;
$$ language plpgsql;

commit;

