require 'sinatra'
require 'pg'
require 'json'

# database connection, to be used in request handling
DB = PG::Connection.new(dbname: 'pig', user: 'pig')

# read the database schema, to reset test data
DIR = File.expand_path('../..', __FILE__)
SQL = File.read(DIR + '/pig.sql')

error Sinatra::NotFound do
  content_type 'application/json'
  [404, '{}']
end

class Pig
  attr_accessor :res

  # prefix every function call with this schema name
  # (database has other schemas. we just one this one)
  def initialize(schema)
    @schema = schema
  end

  # make the "($1,$2)" string for inside exec_params query
  def paramstring(params)
    '(%s)' % (1..params.size).map {|i| "$#{i}"}.join(',')
  end

  # INPUT: function name, varargs parameters
  # OUTPUT: @res['status'] (HTTP status), @res['js'] (JSON)
  def q(func, *params)
    @res = DB.exec_params("SELECT status, js FROM %s.%s%s" %
      [@schema, func, paramstring(params)], params)[0]
  end
end

# before processing every HTTP request...
before do
  # get the apikey from HTTP header. quick fail if none or malformed
  apikey = request.env['HTTP_APIKEY']
  if apikey.nil? || !(/\A[a-z]{4}\Z/ === apikey)
    halt 401, '{"error":"needs apikey header"}'
  end

  # reset the database test info
  DB.exec(SQL)

  # init database connection for use in all handlers
  # 'pig' means every function will be prefixed with that schema name
  @pig = Pig.new('pig')

  # lookup person_id using apikey. remember it if found. fail now if not.
  @pig.q('apikey_get', apikey)
  if '200' == @pig.res['status']
    @person_id = JSON.parse(@pig.res['js'])['person_id']
    @pig.res = nil
  else
    @pig = nil
    halt 401, '{"error":"wrong apikey"}'
  end
end

# nice shortcut instead of manually passing status and js to response
# with each request, the "after" filter does it every time
# the "if" is only because of the halts in the "before" filter
after do
  content_type 'application/json'
  if @pig && @pig.res
    status @pig.res['status']
    body @pig.res['js']
  end
end

# URI route handlers should be self-explanatory
# Note that @person_id from auth stage is used, to limit user to
# only get/set/update/delete their own things

get '/' do
  @pig.q('people_get')
end

get %r{/person/([1-9][0-9]{0,5})} do |id|
  @pig.q('person_get', id)
end

patch '/person' do
  halt(412, '{"error":"missing name"}') unless params[:name]
  @pig.q('person_update', @person_id, params[:name])
end

get '/things' do
  @pig.q('things_get', @person_id)
end

get %r{/thing/([1-9][0-9]{0,5})} do |id|
  @pig.q('thing_get', @person_id, id)
end

patch %r{/thing/([1-9][0-9]{0,5})} do |id|
  halt(412, '{"error":"missing name"}') unless params[:name]
  @pig.q('thing_update', @person_id, id, params[:name])
end

post '/things' do
  halt(412, '{"error":"missing name"}') unless params[:name]
  @pig.q('thing_add', @person_id, params[:name])
end

delete %r{/thing/([1-9][0-9]{0,5})} do |id|
  @pig.q('thing_delete', @person_id, id)
end

