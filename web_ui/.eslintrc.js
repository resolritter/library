const fs = require("fs")
const path = require("path")

const escapeRegExp = (str) => str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")

const getSubDirectoriesRegex = (dir) => {
  const regexGroup = fs
    .readdirSync(dir, {
      withFileTypes: true,
    })
    .map((filePath) =>
      filePath.isDirectory()
        ? filePath.name
        : filePath.name.substr(0, filePath.name.lastIndexOf(".")) ||
          filePath.name,
    )
    .reduce((acc, filePath) => `${acc}|${escapeRegExp(filePath)}`, "")

  return `^(${regexGroup})(\\/.*|$)`
}

const nodeModulesImportRegex = getSubDirectoriesRegex(
  path.join(__dirname, "./node_modules"),
)

const rootModulesImportRegex = getSubDirectoriesRegex(
  path.join(__dirname, "./src"),
)

module.exports = {
  root: true,
  extends: ["eslint:recommended", "plugin:react/recommended"],
  env: {
    browser: true,
    node: true,
    es6: true,
  },
  parser: "babel-eslint",
  parserOptions: {
    ecmaVersion: 6,
    sourceType: "module",
    ecmaFeatures: {
      jsx: true,
    },
  },
  plugins: ["react", "react-hooks", "unused-imports", "simple-import-sort"],
  settings: {
    react: {
      version: "detect",
    },
  },
  rules: {
    "react/prop-types": "off",

    // related to the "unused-imports" plugin
    "react/jsx-uses-react": "warn",
    "react/jsx-uses-vars": "warn",
    "no-unused-vars": "off",
    "unused-imports/no-unused-imports": "error",
    "unused-imports/no-unused-vars": [
      "warn",
      {
        vars: "all",
        varsIgnorePattern: "^_",
        args: "after-used",
        argsIgnorePattern: "^_",
      },
    ],
    "no-multi-spaces": "error",
    "no-multiple-empty-lines": ["error", { max: 1, maxEOF: 1 }],

    // related to the "simple-import-sort" plugin
    "sort-imports": "off",
    "import/order": "off",
    "simple-import-sort/sort": [
      "error",
      {
        groups: [
          ["^react(\\/.*|$)", nodeModulesImportRegex],
          [rootModulesImportRegex],
          [
            "^\\.$",
            // Parent imports
            "^\\.\\.(?!/?$)",
            "^\\.\\./?$",
            // Other relative imports
            "^\\./(?=.*/)(?!/?$)",
            "^\\.(?!/?$)",
            "^\\./?$",
          ],
        ],
      },
    ],

    // misc
    "require-atomic-updates": "off",
    "react/prefer-stateless-function": [
      0,
      {
        ignorePureComponents: false,
      },
    ],
    "use-isnan": "error",
    "no-restricted-syntax": [
      "error",
      {
        selector:
          "CallExpression[callee.name='setTimeout'][arguments.length!=2]",
        message: "setTimeout must always be invoked with two arguments.",
      },
      {
        selector:
          "CallExpression[callee.name='setInterval'][arguments.length!=2]",
        message: "setInterval must always be invoked with two arguments.",
      },
    ],
  },
}
