import { zipObject } from "lodash-es"

import { userAPIAccessLevels } from "src/constants"

export const promiseToSnackbar = function (enqueueSnackbar) {
  return function (prom, onOk, onError) {
    return prom
      .then(function (result) {
        if (result instanceof Error) {
          enqueueSnackbar(result.message, {
            variant: "error",
          })
          if (onError) {
            onError()
          }
        } else if (onOk) {
          onOk()
        }
      })
      .catch(function (err) {
        enqueueSnackbar(err.message, {
          variant: "error",
        })
      })
  }
}

export const dictionaryOf = function (array) {
  return zipObject(array, array)
}

export const loadingStates = dictionaryOf(["notStarted", "loading", "loaded"])

export const getAccessMaskDisplayName = function (mask) {
  for (const name in userAPIAccessLevels) {
    if (userAPIAccessLevels[name] === mask) {
      return name
    }
  }
}
