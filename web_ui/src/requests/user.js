import { getReasonPhrase, StatusCodes } from "http-status-codes"
import { apiEndpoints, userAPIAccessLevels } from "src/constants"
import { store } from "src/setup"
import userStore from "src/store/user"

import { getAccessToken, getCors } from "./index"

export const createUser = async function ({ email, accessLevel }) {
  try {
    let accessMask
    if (accessLevel) {
      const accessMask = userAPIAccessLevels[accessLevel]
      if (!accessMask) {
        return new Error(`${accessLevel} is not a valid permission`)
      }
    } else {
      accessMask = userAPIAccessLevels.user
    }

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
      store.dispatch(userStore.actions.setUser(await response.json()))
      return { status: "ok" }
    } else {
      try {
        return new Error((await response.json()).message)
      } catch (err) {
        return new Error(getReasonPhrase(response.status))
      }
    }
  } catch (err) {
    return err
  }
}
