import { zipObject } from "lodash-es"

const cssVariablePrefix = "--"

export const handleWithSnackbar = function (enqueueSnackbar) {
  return function (prom, continuation) {
    prom
      .then(function (result) {
        if (result instanceof Error) {
          enqueueSnackbar(result.message, {
            variant: "error",
          })
        } else {
          continuation()
        }
      })
      .catch(function (err) {
        enqueueSnackbar(err.message, {
          variant: "error",
        })
      })
  }
}

export const newCssVariableName = function (name) {
  return `${cssVariablePrefix}${name}`
}

export const cssVariableValueOf = function (name) {
  return `var(${newCssVariableName(name)})`
}

export const dictionaryOf = function (array) {
  return zipObject(array, array)
}

export const loadingStates = dictionaryOf(["notStarted", "loading", "loaded"])

export const setCssVariable = function (name, value) {
  document.documentElement.style.setProperty(newCssVariableName(name), value)
}
