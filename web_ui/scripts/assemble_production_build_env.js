const path = require("path")
const fs = require("fs")
const envDir = path.resolve(__dirname, "../env")
const crypto = require("crypto")

if (!fs.existsSync(envDir)) {
  throw new Error(`Environment directory ${envDir} missing.`)
}

var extraContent = "\n"
const envParts = path.join(envDir, "webpack.prod.build.env.part")
if (fs.existsSync(envParts)) {
  extraContent = `${fs.readFileSync(envParts)}\n`
}

const randomSalt = crypto.randomBytes(32).toString("base64")

const targetPath = path.join(envDir, "webpack.prod.build.env")

fs.writeFileSync(targetPath, `CHUNKS_HASH_SALT=${randomSalt}\n${extraContent}`)

console.log(`Production environment assembled to ${targetPath}`)
