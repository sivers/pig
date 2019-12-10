require 'sinatra'
require 'pg'
require 'json'
DB = PG::Connection.new(dbname: 'pig', user: 'pig')
SCHEMA = File.read('../pig.sql')

class Pig
  attr_reader :res

  def initialize(schema)
    @schema = schema
  end

  # make the "($1,$2)" string for inside exec_params query
  def paramstring(params)
    '(%s)' % (1..params.size).map {|i| "$#{i}"}.join(',')
  end

  # query database & use @res['status'] and @res['js'] below
  def q(func, *params)
    @res = DB.exec_params("SELECT status, js FROM %s.%s%s" %
      [@schema, func, paramstring(params)], params)[0]
  end
end

before do
  # reset the database before every request
  DB.exec(SCHEMA)
  @pig = Pig.new('pig')
  # TODO: get it from HTTP Request
  apikey = 'aaaa'
  @pig.q('apikey_get', apikey)
  if '200' == @pig.res['status']
    @person_id = JSON.parse(@pig.res['js'])['person_id']
  else
    halt 401, 'bad API key'
  end
end

after do
  content_type 'application/json'
  status @pig.res['status']
  body @pig.res['js']
end

get '/' do
  @pig.q('people_get')
end

get %r{/person/([1-9][0-9]{0,5})} do |id|
  @pig.q('person_get', id)
end

patch '/person' do
  @pig.q('person_update', @person_id, params[:name])
end

get '/things' do
  @pig.q('things_get', @person_id)
end

get %r{/thing/([1-9][0-9]{0,5})} do |id|
  @pig.q('thing_get', @person_id, id)
end

patch %r{/thing/([1-9][0-9]{0,5})} do |id|
  @pig.q('thing_update', @person_id, id, params[:name])
end

post '/things' do
  @pig.q('thing_add', @person_id, params[:name])
end

delete %r{/thing/([1-9][0-9]{0,5})} do |id|
  @pig.q('thing_delete', @person_id, id)
end

