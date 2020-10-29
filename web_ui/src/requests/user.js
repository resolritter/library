import { StatusCodes } from "http-status-codes"

import { getAccessToken, getCors, handleErrorResponse } from "."

import { apiEndpoints, userAPIAccessLevels } from "src/constants"
import { store } from "src/setup"
import userStore from "src/store/user"

export const login = async function ({ email }) {
  const response = await fetch(apiEndpoints.session(), {
    method: "POST",
    mode: getCors(),
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      email,
    }),
  })

  if (response.status === StatusCodes.CREATED) {
    store.dispatch(userStore.actions.setUser(await response.json()))
  } else {
    return await handleErrorResponse(response)
  }
}

export const createUser = async function ({
  email,
  accessLevel,
  shouldSetAsCurrent,
}) {
  const accessMask = accessLevel
    ? userAPIAccessLevels[accessLevel]
    : userAPIAccessLevels.user

  const response = await fetch(apiEndpoints.createUser(), {
    method: "POST",
    mode: getCors(),
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      email,
      access_mask: accessMask,
      requester_access_token: getAccessToken(),
    }),
  })

  if (response.status === StatusCodes.CREATED) {
    if (shouldSetAsCurrent) {
      store.dispatch(userStore.actions.setUser(await response.json()))
    }
  } else {
    return await handleErrorResponse(response)
  }
}
