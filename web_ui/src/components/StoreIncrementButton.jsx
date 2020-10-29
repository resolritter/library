import { get } from "lodash/fp"
import React from "react"
import { useDispatch, useSelector } from "react-redux"
import { increment } from "src/store/counter"

import IncrementButton from "./IncrementButton"

export const StoreIncrementButton = function () {
  const count = useSelector(get("counter"))
  const dispatch = useDispatch()
  return (
    <IncrementButton
      increment={function () {
        dispatch(increment())
      }}
      count={count}
      title={"Increment counter (Redux state)"}
    />
  )
}

export default StoreIncrementButton
