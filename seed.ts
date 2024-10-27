export {};

async function main() {
  var res = await fetch("localhost:3000/signup", {
    body: JSON.stringify({
      name: "aster-void",
      password: "hashed-password",
    }),
  });
  var res = await fetch("localhost:3000/login", {
    body: JSON.stringify({
      name: "aster-void",
      password: "hashed-password",
    }),
  });
  const token = (await res.json()).token;
  var res = await fetch("localhost:3000/workspace", {
    body: JSON.stringify({
      name: "new workspace",
      public: true,
    }),
    headers: {
      "Auth-Token": token,
    },
  });

  const workspaceId = (await res.json()).id;
  var res = await fetch("localhost:3000/$wsid/chat", {
    body: JSON.stringify({
      name: "new channel",
    }),
    headers: {
      "Auth-Token": token,
    },
  });
  const channelId = (await res.json()).id;
  var res = await fetch("localhost:3000/$wsid/chat/$chanid/send", {
    body: JSON.stringify({
      content: "Hello World!",
    }),
    headers: {
      "Auth-Token": token,
    },
  });
}

await main();
