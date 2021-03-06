module.exports = {
  presets: [
    ["@babel/preset-env", { targets: { browsers: ["last 2 versions"] } }],
    "@babel/preset-react",
    "linaria/babel",
  ],
  plugins: [["@babel/plugin-transform-runtime"]],
}
