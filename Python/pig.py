import psycopg2
from flask import Flask
import os


app = Flask(__name__)

DB = psycopg2.connect("dbname=pig user=pig")

DIR = os.path.abspath(".")
with open(DIR + '/pig.sql') as f:
    SQL = f.read()
