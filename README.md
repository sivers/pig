REST API webservers, getting JSON and status from PostgreSQL

# INSTALL:

```
# as your PostgreSQL super-user:
createuser -s pig
createdb -U pig pig
psql -U pig -d pig -f pig.sql
```

## GO:
[Download](https://golang.org/dl/) and Install Go (minimum) version 1.12.* 

```
GO111MODULE=on go mod download
go run *.go
```

## Ruby:

```
gem install pg sinatra thin minitest
ruby Ruby/pig.rb
```

## Test:

```
sh test.sh http://127.0.0.1:4567/
```

# WRITE YOURS:

### 1. PostgreSQL query-maker

Goal: to make all queries simple and succinct, using variadic arguments.

Avoid repetition since every query is *select status, js from schema.function(params)*

See lines 17-37 of the Ruby file for an example.

1. Init with the schema name, returning curried function or object to be used from then on.
2. paramstring function takes variadic arguments and returns query param string:
	* no arguments returns "()"
	* one argument returns "($1)"
	* two arguments returns "($1, $2)"
	* three arguments returns "($1, $2, $3)"
	* … etc.
	* Ignore the actual arguments, just count them to make the strings described above.
3. query ("q") function takes the function name and variadic arguments to build the SQL string, for example:
	* db = Pig.new('cow')
	* db.q('foo') makes "select status, js from cow.foo()"
	* db.q('foo', 9) makes "select status, js from cow.foo($1)", [9]
	* db.q('foo', 9, 'b') makes "select status, js from cow.foo($1, $2)", [9, 'b']
	* … then queries the PostgreSQL database, and return the result into a variable used in the next step.

### 2. Every HTTP response

See lines 68-74 of the Ruby file for an example.

1. Every HTTP response has content type 'application/json'
2. Every HTTP response uses "status" response from database as the HTTP status code…
3. … unless it stops before touching the database, in the next authentication section
4. Every HTTP response uses "js" JSON response from database as the body.

### 3. HTTP request authentication

See lines 40-63 of the Ruby file for an example.

1. Before every incoming HTTP request, look for "apikey" in request header.
2. If none, or doesn't match \A[a-z]{4}\Z regex, stop with HTTP 401 + body {"error":"needs apikey header"}. Doesn't have to be PCRE. Any simple pattern matching will do.
3. Query database: function name="apikey\_get", argument = apikey from step 1.
4. If database response had status other than "200", stop with HTTP 401 + body {"error":"wrong apikey"}
5. Get person\_id from database ("js") response and save it as person\_id for next section.

### 4. URL routes

See lines 80-113 of the Ruby file for an example.

1. Routes should be simple, clean, succinct.  If too verbose, try to wrap in a function to simplify.
2. Request methods GET, POST, PUT, PATCH, and DELETE are needed. But no others.
3. Routes can have pattern matching. This prevents wrong IDs like "0", "x'x", or even "999999999" from being passed to the database.  See line 84 of the Ruby file for an example. Doesn't have to be PCRE. Any simple pattern matching will do.
4. To save unnecessary database connections, routes can be halted with an error if a parameter is missing.  See line 89 of the Ruby file for an example.
5. Routes extrapolate variables from the URL or posted params, to be used in next step.
6. Every route calls a PostgreSQL function using your query-maker from section 1, passing the values from the URL, parameters, or person\_id from section 3's authentication, which then sends the HTTP response from section 2.

That's it!

The Ruby example code here works perfectly, so I'm curious to see how it would work in other languages. 

If you want to write one, please:

1. fork
2. make a subdirectory with the name of your language (like "Ruby/" and "Rust/" here) with all code inside
3. update this README.md to add the INSTALL instructions for your language, like lines 12-17 of this README.md
4. pull request

