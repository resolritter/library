const path = require("path")
const fs = require("fs")
const cp = require("child_process")
const assert = require("assert").strict
const sleep = require("sleep")
const chokidar = require("chokidar")
const tempy = require("tempy")

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

const installWatcher = function () {
  const signalFile = tempy.file()
  const whenReady = new Promise(function (resolve) {
    const fileName = path.basename(signalFile)
    const watcher = chokidar.watch(signalFile, { persistent: true })

    watcher.on("raw", function (event, someFileName) {
      if (someFileName === fileName) {
        watcher.close()
        setTimeout(resolve, 1000)
      }
    })
  })
  return { signalFile, whenReady }
}

const serverPort = getFreePort()
const uiPort = getFreePort()

const {
  signalFile: serverSignalFile,
  whenReady: whenServerReady,
} = installWatcher()
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
    "--signal-file",
    serverSignalFile,
    "--port",
    serverPort,
    "test_server",
  ],
  { stdio: ["ignore", "inherit", "inherit"] },
)

const uiDir = path.join(__dirname, "..")
const uiAddr = `http://127.0.0.1:${uiPort}`
const { signalFile: uiSignalFile, whenReady: whenUiReady } = installWatcher()
const ui = cp.spawn(
  "bash",
  [
    "-c",
    `PORT=${uiPort} SIGNAL_FILE="${uiSignalFile}" API_URL="${serverAddr}" npm run --prefix "${uiDir}" start`,
  ],
  {
    stdio: ["ignore", "inherit", "inherit"],
  },
)

const cleanOnExit = function () {
  server.kill()
  ui.kill()
  cp.execFileSync(executablePath, [
    "free_port",
    uiPort,
    {
      stdio: ["ignore", "inherit", "inherit"],
    },
  ])
  process.exit()
}
process.on("SIGINT", cleanOnExit)
process.on("SIGTERM", cleanOnExit)

const operation = process.argv[2] || "run"
const cypressPath = path.join(__dirname, "./node_modules/.bin/cypress")

whenServerReady.then(function () {
  whenUiReady.then(function () {
    cp.spawnSync(
      cypressPath,
      [operation, "--env", `UIAddr=${uiAddr},ID=${uiPort}`],
      { cwd: __dirname, stdio: ["ignore", "inherit", "inherit"] },
    )
    cleanOnExit()
  })
})
