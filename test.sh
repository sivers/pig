#!/bin/sh
# USAGE: PASS THE BASE URL/HOST
# sh test.sh http://127.0.0.1:4567/
# or
# sh test.sh https://cloudfr.ee/
host=$1

# TODO: put into a real assert, in a scripting language

echo "ASSERT: 401 needs apikey header"
curl -i $host
echo

echo "ASSERT: 401 needs apikey header"
curl -i -H "apikey: x" $host
echo

echo "ASSERT: 401 wrong apikey"
curl -i -H "apikey: xxxx" $host
echo

echo "ASSERT: 200 + list of people, Ada and Bai"
curl -i -H "apikey: aaaa" $host
echo

echo "ASSERT: person 2 : Bai"
curl -H "apikey: aaaa" "${host}person/2"
echo

echo "ASSERT: 404 Not Found"
curl -i -H "apikey: aaaa" "${host}person/99"
echo

echo "ASSERT: 404 Not Found"
curl -i -H "apikey: aaaa" "${host}nosuchurl"
echo

echo "ASSERT: 412 Precondition Failed (missing name)"
curl -i -H "apikey: bbbb" --request PATCH "${host}person"
echo

echo "ASSERT: new name id 2 is Bob"
curl -H "apikey: bbbb" --request PATCH --data "name=Bob" "${host}person"
echo

echo "ASSERT: new name id 1 is Alice"
curl -H "apikey: aaaa" --request PATCH --data "name=Alice" "${host}person"
echo

echo "ASSERT: person 1 things (1 + 2)"
curl -H "apikey: aaaa" "${host}things"
echo

echo "ASSERT: person 2 things (3 + 4)"
curl -H "apikey: bbbb" "${host}things"
echo

echo "ASSERT: person 1 can see thing 1"
curl -H "apikey: aaaa" "${host}thing/1"
echo

echo "ASSERT: 404 person 1 can NOT see thing 4"
curl -i -H "apikey: aaaa" "${host}thing/4"
echo

echo "ASSERT: person 2 can see thing 4"
curl -H "apikey: bbbb" "${host}thing/4"
echo

echo "ASSERT: 412 Precondition Failed (missing name)"
curl -i -H "apikey: aaaa" --request PATCH "${host}thing/1"
echo

echo "ASSERT: 500 Server Error empty name violates constraint thing_name"
curl -i -H "apikey: aaaa" --request PATCH --data "name=" "${host}thing/1"
echo

echo "ASSERT: item 1 new name: couch"
curl -H "apikey: aaaa" --request PATCH --data "name=couch" "${host}thing/1"
echo

echo "ASSERT: 412 Precondition Failed (missing name)"
curl -i -H "apikey: aaaa" --request POST "${host}things"
echo

echo "ASSERT: new thing id:5 name:gum"
curl -H "apikey: aaaa" --request POST --data "name=gum" "${host}things"
echo

echo "ASSERT: 404 person 1 can NOT delete thing 4"
curl -i -H "apikey: aaaa" --request DELETE "${host}thing/4"
echo

echo "ASSERT: person 2 can delete thing 4"
curl -H "apikey: bbbb" --request DELETE "${host}thing/4"
echo

