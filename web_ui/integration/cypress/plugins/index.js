const preprocessor = require("@cypress/webpack-preprocessor")
module.exports = (on, config) => {
  const options = {
    webpackOptions: {
      resolve: {
        extensions: [".ts", ".js", ".json"],
      },
      module: {
        rules: [
          {
            test: /\.ts$/,
            loader: "ts-loader",
            options: { transpileOnly: true },
          },
        ],
      },
    },
  }
  on("file:preprocessor", preprocessor(options))
}
