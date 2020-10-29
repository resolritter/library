import { getReasonPhrase } from "http-status-codes"

import { apiURL, devAPIURL } from "src/constants"
import { store } from "src/setup"

export const getCors = function () {
  return apiURL == devAPIURL ? "cors" : "no-cors"
}

export const getAccessToken = function () {
  return store.getState().user.accessToken
}

export const handleErrorResponse = async function (response) {
  try {
    return new Error((await response.json()).message)
  } catch (err) {
    return new Error(getReasonPhrase(response.status))
  }
}
