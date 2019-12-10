import httpClient
var client = newHttpClient()
let base = "http://127.0.0.1:4567/"
echo client.getContent(base)
echo client.getContent(base & "things")
echo client.getContent(base & "person/2")
var res = client.request(base & "person/9")
echo res.status
#[
assert res == """[{"id":1,"name":"Ada"},
 {"id":2,"name":"Bai"}]"""
]#
