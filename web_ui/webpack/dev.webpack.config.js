const path = require("path")
const MiniCssExtractPlugin = require("mini-css-extract-plugin")
const fs = require("fs")
const ser = require("serialize-javascript")

const webpack = require("webpack")
const HtmlWebpackPlugin = require("html-webpack-plugin")

const baseConfiguration = require("./base.webpack.config")
const { mergeConfigurations } = require("./utils")

const plugins = [
  new webpack.EnvironmentPlugin(["API_URL"]),
  new MiniCssExtractPlugin({
    filename: "styles.css",
  }),
  new HtmlWebpackPlugin({
    inject: true,
    template: "index.html",
  }),
  new webpack.HotModuleReplacementPlugin(),
]

module.exports = mergeConfigurations(baseConfiguration, {
  devtool: "inline-source-map",
  output: {
    path: path.join(baseConfiguration.context, "build"),
    filename: "[name].js",
    publicPath: "/",
  },
  devServer: {
    host: "localhost",
    port: process.env.PORT || "3000",
    clientLogLevel: "none",
    stats: "errors-only",
    historyApiFallback: true,
    hot: true,
    contentBase: path.join(baseConfiguration.context, "assets"),
    onListening: (server) => {
      server.compiler.hooks.done.tap("done", () => {
        setImmediate(() => {
          if (process.env.SIGNAL_FILE) {
            fs.writeFileSync(process.env.SIGNAL_FILE, "READY")
          }
        })
      })
    },
  },
  plugins,
})
