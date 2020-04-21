import psycopg2
from flask import Flask, jsonify
import os


app = Flask(__name__)

conn = psycopg2.connect("dbname=pig user=pig")
conn.autocommit = True
DB = conn.cursor() 

DIR = os.path.abspath(".")
with open(DIR + '/pig.sql') as f:
    SQL = f.read()

@app.errorhandler(404)
def not_found(error):
    return jsonify(404, "{}")

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
        self.res = DB.fetchall()[0]

if __name__ == "__main__":
    app.run(debug=True)
