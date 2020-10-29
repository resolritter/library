import { createSlice } from "@reduxjs/toolkit"

const storageKey = "user__profile"
const storageState = window.localStorage.getItem(storageKey)

export default createSlice({
  name: "user",
  initialState: storageState ? JSON.parse(storageState) : {},
  reducers: {
    setUser: function (state, { payload: profile }) {
      if (profile) {
        window.localStorage.setItem(storageKey, JSON.stringify({ profile }))
      } else {
        window.localStorage.removeItem(storageKey)
      }
      state.profile = profile
    },
  },
})
