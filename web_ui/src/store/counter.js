import { createSlice } from "@reduxjs/toolkit"

export const initialState = 0

const slice = createSlice({
  name: "counter",
  initialState,
  reducers: {
    increment: function (state, { payload = 1 }) {
      return state + payload
    },
  },
})

export const { increment } = slice.actions

export default slice
