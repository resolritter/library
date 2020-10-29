import React from "react"

import { themedButton } from "./style"

export const IncrementButton = function ({
  increment,
  count,
  title = "Increment counter",
}) {
  return (
    <div>
      <span>{count}</span>
      <button className={themedButton} onClick={increment}>
        {title}
      </button>
    </div>
  )
}

export default IncrementButton
