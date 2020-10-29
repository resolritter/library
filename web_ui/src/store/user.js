import { createSlice } from "@reduxjs/toolkit"

export const initialState = {}

export default createSlice({
  name: "user",
  initialState,
  reducers: {
    setUser: function (state, { payload: profile }) {
      return profile
    },
  },
})
