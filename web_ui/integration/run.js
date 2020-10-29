const path = require("path")
const fs = require("fs")
const cp = require("child_process")
const assert = require("assert").strict
const sleep = require("sleep")

const testsLockPath = path.join(__dirname, ".tests.lock")
const executablePath = path.join(__dirname, "../../run.sh")

const getFreePort = function () {
  const port = cp
    .execFileSync("flock", [
      "-x",
      testsLockPath,
      "-c",
      `"${executablePath}" get_port_sync | tail -n +2`,
    ])
    .toString()
    .trim()
  assert(!isNaN(parseInt(port)))
  return port
}

const serverPort = getFreePort()
const webUIPort = getFreePort()

const serverAddr = `http://127.0.0.1:${serverPort}`
const server = cp.spawn(
  executablePath,
  [
    "--listen",
    serverAddr,
    "--port",
    serverPort,
    "--instance",
    `${serverPort}__${Date.now()}`,
    "--admin-credentials-for-test",
    "admin@admin.com",
    "--port",
    serverPort,
    "test_server",
  ],
  { stdio: ["ignore", "inherit", "inherit"] },
)

const webUIDir = path.join(__dirname, "..")
const webUIAddr = `http://127.0.0.1:${webUIPort}`
const webUI = cp.spawn(
  "bash",
  [
    "-c",
    `PORT=${webUIPort} API_URL="${serverAddr}" npm run --prefix "${webUIDir}" start`,
  ],
  {
    stdio: ["ignore", "inherit", "inherit"],
  },
)

const cleanOnExit = function () {
  server.kill()
  webUI.kill()
  cp.execFileSync(executablePath, [
    "free_port",
    webUIPort,
    {
      stdio: ["ignore", "inherit", "inherit"],
    },
  ])
  process.exit()
}
process.on("SIGINT", cleanOnExit)
process.on("SIGTERM", cleanOnExit)

const specName = process.argv[2]
const cypressPath = path.join(__dirname, "./node_modules/.bin/cypress")

sleep.msleep(5000)
const runCypress = cp.spawnSync(
  cypressPath,
  ["open", "--env", `UIAddr=${webUIAddr},ID=${webUIPort}`],
  { cwd: __dirname, stdio: ["ignore", "inherit", "inherit"] },
)
cleanOnExit()
