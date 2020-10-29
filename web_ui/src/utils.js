import { zipObject } from "lodash-es"

const cssVariablePrefix = "--"

export const newCssVariableName = function (name) {
  return `${cssVariablePrefix}${name}`
}

export const cssVariableValueOf = function (name) {
  return `var(${newCssVariableName(name)})`
}

export const dictionaryOf = function (array) {
  return zipObject(array, array)
}

export const setCssVariable = function (name, value) {
  document.documentElement.style.setProperty(newCssVariableName(name), value)
}
