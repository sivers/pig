REST API webservers, getting JSON and status from PostgreSQL

# INSTALL:

```
# as your PostgreSQL super-user:
createuser -s pig
createdb -U pig pig
psql -U pig -d pig -f pig.sql
```

## Ruby:

```
gem install pg sinatra thin minitest
ruby Ruby/pig.rb
```

