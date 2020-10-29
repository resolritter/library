const { isArray, mergeWith } = require("lodash")
const crypto = require("crypto")

const chunkSalt = process.env["CHUNKS_HASH_SALT"]

if (process.env["NODE_ENV"] === "production" && !chunkSalt) {
  console.error(
    `Salt ${chunkSalt} for production build chunks is not set. Exiting!`,
  )
  process.exit(1)
}

module.exports = {
  mergeConfigurations(baseConfiguration, otherConfiguration) {
    return mergeWith(baseConfiguration, otherConfiguration, function (
      objValue,
      srcValue,
    ) {
      if (isArray(objValue)) {
        return objValue.concat(srcValue)
      }
    })
  },
  hashChunk(chunkName) {
    const hashBuffer = crypto.pbkdf2Sync(chunkName, chunkSalt, 4, 16, "md5")
    return hashBuffer.toString("hex")
  },
}
