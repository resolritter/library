import { zipObject } from "lodash-es"

import { dictionaryOf } from "src/utils"

export const minute = 60
export const hour = 60 * minute
export const day = 24 * hour
export const week = 7 * day

export const devAPIURL = "http://localhost:8080"
export const apiURL = process.env.API_URL ?? devAPIURL

const userUIAccessLevelsList = ["", "librarian", "admin"]
const userUIAccessLevelsListNamed = ["user", ...userUIAccessLevelsList.slice(1)]
export const userUIAccessLevels = dictionaryOf(userUIAccessLevelsListNamed)
export const userAPIAccessLevels = zipObject(userUIAccessLevelsListNamed, [
  0x001,
  0x011,
  0x11,
])

export const apiEndpoints = {
  createUser: function () {
    return `${apiURL}/user`
  },
  borrowBook: function ({ title }) {
    return `${apiURL}/book/${title}/borrow`
  },
  session: function () {
    return `${apiURL}/session`
  },
  books: function () {
    return `${apiURL}/books`
  },
}

export const routes = {
  home: function () {
    return "/"
  },
  login: function () {
    return "/login"
  },
  createUser: function () {
    return "/create_user"
  },
}
