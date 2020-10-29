import { createSlice } from "@reduxjs/toolkit"
import React, { useReducer } from "react"

import IncrementButton from "./IncrementButton"

export const initialState = 0
const slice = createSlice({
  name: "LocalIncrementButton",
  initialState,
  reducers: {
    increment: function (state, { payload = 1 }) {
      return state + payload
    },
  },
})

const { increment } = slice.actions

export function LocalIncrementButton() {
  const [count, thisDispatcher] = useReducer(slice.reducer, initialState)
  return (
    <IncrementButton
      increment={function () {
        thisDispatcher(increment())
      }}
      count={count}
      title={"Increment counter (local state)"}
    />
  )
}

export default LocalIncrementButton
