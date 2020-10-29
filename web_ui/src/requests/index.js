import { getReasonPhrase } from "http-status-codes"

import { store } from "src/setup"

export const getCors = function () {
  return "cors"
}

export const getAccessToken = function () {
  return store.getState().user.profile?.access_token
}

export const handleErrorResponse = async function (response) {
  try {
    const body = await response.json()
    if (body.Err) {
      return new Error(body.Err)
    }
  } catch {
    return new Error(getReasonPhrase(response.status))
  }
  return new Error(getReasonPhrase(response.status))
}
