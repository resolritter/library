;(async function() {
  const memcached = require("memcache-client")
  const m = new memcached({ server: "localhost:11211" })
  const key = process.argv.slice(2)[0]
  if (!key) {
    console.error("Key is empty")
    process.exit(1)
  }
  const value = await m.get(key)
  if (value) {
    console.log("taken")
    process.exit(0)
  }
  await m.set(key, "b")
  process.exit(0)
})()
