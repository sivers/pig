import psycopg2
from flask import Flask, jsonify, request, abort
import os
import psycopg2.extras
import re
from functools import wraps

app = Flask(__name__)

conn = psycopg2.connect("dbname=pig user=pig")
conn.autocommit = True
DB = conn.cursor(cursor_factory=psycopg2.extras.DictCursor) 

DIR = os.path.abspath(".")
with open(DIR + '/pig.sql') as f:
    SQL = f.read()

@app.errorhandler(404)
def not_found(error):
    return jsonify(404, "{}")

@app.errorhandler(401)
def missing_apikey(error):
    return jsonify(401, "{'error': 'needs apikey header'}")

class Pig:
    def __init__(self, schema):
        self.schema = schema

    def format_parameter(self, num):
        return f"%s"

    def paramstring(self, num):
        list_of_nums = list(range(1,num+1))
        joined_nums = map(self.format_parameter, list_of_nums)
        return f"({','.join(joined_nums)})"

    def q(self, func, *params):
        DB.execute(f"SELECT status, js FROM {self.schema}.{func}{self.paramstring(len(params))}", params) 
        self.res = DB.fetchone()

def before():
    print('test wrapper')
    apikey = request.headers.get('Apikey')
    print(apikey)
    print(apikey is None)
    print(re.search("\A[a-z]{4}\Z", apikey))
    print((apikey is None) or (re.search("\A[a-z]{4}\Z", apikey) is None))
    if (apikey is None) or (re.search("\A[a-z]{4}\Z", apikey) is None):
        abort(401)

    pig_ = Pig('pig')

def after():
    print('after')
    pass

def before_and_after():
    def decorator_func(func):
        @wraps(func)
        def wrapper_func(*args, **kwargs):
            before()
            retval = func(*args, **kwargs)
            after()
            return jsonify('test')
        return wrapper_func
    return decorator_func

@app.route('/')
@before_and_after()
def people_get():
    pig_.q('people_get')

if __name__ == "__main__":
    app.run(debug=True)
