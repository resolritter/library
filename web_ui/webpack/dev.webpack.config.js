const path = require("path")
const MiniCssExtractPlugin = require("mini-css-extract-plugin")

const webpack = require("webpack")
const HtmlWebpackPlugin = require("html-webpack-plugin")

const baseConfiguration = require("./base.webpack.config")
const { mergeConfigurations } = require("./utils")

module.exports = mergeConfigurations(baseConfiguration, {
  devtool: "inline-source-map",
  output: {
    path: path.join(baseConfiguration.context, "build"),
    filename: "[name].js",
    publicPath: "/",
  },
  devServer: {
    host: "localhost",
    port: "3000",
    clientLogLevel: "none",
    open: true,
    stats: "errors-only",
    historyApiFallback: true,
    hot: true,
    contentBase: path.join(baseConfiguration.context, "assets"),
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: "styles.css",
    }),
    new HtmlWebpackPlugin({
      inject: true,
      template: "index.html",
    }),
    new webpack.HotModuleReplacementPlugin(),
  ],
})
