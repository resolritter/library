import { apiURL, devAPIURL } from "src/constants"
import { store } from "src/setup"

export const getCors = function () {
  return apiURL == devAPIURL ? "cors" : "no-cors"
}

export const getAccessToken = function () {
  return store.getState().user.accessToken
}
