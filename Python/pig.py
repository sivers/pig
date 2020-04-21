import psycopg2
from flask import Flask, jsonify, request, abort, Response
import os
import psycopg2.extras
import re
from functools import wraps
from werkzeug.routing import BaseConverter

app = Flask(__name__)


class RegexConverter(BaseConverter):
    def __init__(self, url_map, *items):
        super(RegexConverter, self).__init__(url_map)
        self.regex = items[0]


app.url_map.converters["regex"] = RegexConverter
conn = psycopg2.connect("dbname=pig user=pig")
conn.autocommit = True
DB = conn.cursor(cursor_factory=psycopg2.extras.DictCursor)

DIR = os.path.abspath(".")
with open(DIR + "/pig.sql") as f:
    SQL = f.read()


@app.errorhandler(404)
def not_found(error):
    return jsonify("{}"), 404


class MissingApikey(Exception):
    pass


@app.errorhandler(MissingApikey)
def missing_apikey(error):
    return jsonify("{'error': 'needs apikey header'}"), 401


class WrongApikey(Exception):
    pass


@app.errorhandler(WrongApikey)
def wrong_apikey(error):
    return jsonify("{'error':'wrong apikey'}"), 401

class MissingName(Exception):
    pass

@app.errorhandler(MissingName)
def missing_name(error):
    return jsonify("{'error':'missing name'}"), 412


class Pig:
    def __init__(self, schema):
        self.schema = schema

    def format_parameter(self, num):
        return f"%s"

    def paramstring(self, num):
        list_of_nums = list(range(1, num + 1))
        joined_nums = map(self.format_parameter, list_of_nums)
        return f"({','.join(joined_nums)})"

    def q(self, func, *params):
        DB.execute(
            f"SELECT status, js FROM {self.schema}.{func}{self.paramstring(len(params))}",
            params,
        )
        self.res = DB.fetchone()


def before():
    apikey = request.headers.get("Apikey")
    if (apikey is None) or (re.search("\A[a-z]{4}\Z", apikey) is None):
        raise MissingApikey

    DB.execute(SQL)

    pig_ = Pig("pig")

    pig_.q("apikey_get", apikey)
    print(pig_.res)
    if 200 == pig_.res["status"]:
        pig_.person_id = pig_.res["js"].get("person_id")
        pig_.res = None
    else:
        pig_.res = None
        raise WrongApikey

    return pig_


def after(pig_):
    if pig_ and pig_.res:
        return jsonify(pig_.res["js"]), pig_.res["status"]


def before_and_after():
    def decorator_func(func):
        @wraps(func)
        def wrapper_func(*args, **kwargs):
            pig_ = before()
            retval = func(pig_, *args, **kwargs)
            return after(pig_)

        return wrapper_func

    return decorator_func


@app.route("/")
@before_and_after()
def people_get(pig_):
    pig_.q("people_get")


@app.route("/person/<regex('[1-9][0-9]{0,5}'):id>")
@before_and_after()
def person_get(pig_, id):
    pig_.q("person_get", id)

@app.route("/person", methods=["PATCH"])
@before_and_after()
def person_update(pig_):
    if request.form.get("name") is None:
        raise MissingName    
    pig_.q("person_update", pig_.person_id, request.form.get("name"))


if __name__ == "__main__":
    app.run(debug=True)
