curl localhost:3000/signup -d '{"name":"aster-void","password":"hashed-password"}'
curl localhost:3000/login -d '{"name":"aster-void","password":"hashed-password"}'
token='1'
curl -X POST localhost:3000/workspace -d '{"name":"new workspace","public":true}' -H "Auth-Token: $token"
wsid='1'
curl -X POST localhost:3000/$wsid/chat -d '{"name":"new channel"}' -H "Auth-Token: $token"
chanid='1'
curl -X POST localhost:3000/$wsid/chat/$chanid/send -d '{"content":"Hello World!"}' -H "Auth-Token: $token"

