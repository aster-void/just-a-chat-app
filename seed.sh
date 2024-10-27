#!/usr/bin/env nu

http post http://localhost:3000/signup -t application/json {
  name: "aster-void"
  password: "hashed-password"
};

let token: string = http post http://localhost:3000/login -t application/json {
  name: "aster-void"
  password: "hashed-password"
} | get token;
print $"token is ($token)";

let wsid: string = http post http://localhost:3000/workspace -H [Auth-Token, $token] -t application/json {
  name:"new workspace"
  public: true
} | get id;
print $"workspace id is ($wsid)";

let chanid: string = http post $"http://localhost:3000/($wsid)/chat" -H [Auth-Token, $token] -t application/json { 
  "name": "new channel"
} | get id;
print "channel id is ($chanid)";

http post $"http://localhost:3000/($wsid)/chat/($chanid)/send" -H [Auth-Token, $token] -t application/json {
  "content":"Hello World!"
}

