import React from "react"
import { getActiveTheme, setTheme, themeEntries } from "src/theme"

import { themedButton } from "./style"

export const ChangeThemeButton = function () {
  return (
    <button
      className={themedButton}
      onClick={function () {
        const activeTheme = getActiveTheme()
        if (activeTheme == "light") {
          setTheme(themeEntries.dark)
        } else {
          setTheme(themeEntries.light)
        }
      }}
    >
      Change the theme
    </button>
  )
}

export default ChangeThemeButton
