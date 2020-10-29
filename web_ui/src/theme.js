import { zipObject } from "lodash-es"
import { useEffect, useState } from "react"
import { useId } from "react-id-generator"
import { setCssVariable } from "src/utils"

export const themes = {
  light: {
    name: "light",
    theme: {
      backgroundColor: "white",
      color: "black",
    },
  },
  dark: {
    name: "dark",
    theme: {
      backgroundColor: "black",
      color: "white",
    },
  },
}
export const themeList = Object.keys(themes)
export const themeEntries = zipObject(themeList, themeList)
export const initialTheme = "light"

let activeTheme
export const getActiveTheme = function () {
  return activeTheme
}

const themeChangedListeners = new Map()
export const useTheme = function () {
  const id = useId()[0]
  const [theme, setTheme] = useState(themes[getActiveTheme()].theme)
  useEffect(function () {
    themeChangedListeners.set(id, function subscription(newTheme) {
      setTheme(themes[newTheme])
    })

    return function unsubscribe() {
      themeChangedListeners.delete(id)
    }
  }, [])
  return theme
}

export const setTheme = function (name) {
  activeTheme = name
  const theme = themes[activeTheme].theme
  Object.keys(theme).forEach(function (key) {
    setCssVariable(key, theme[key])
  })

  activeTheme = name
  themeChangedListeners.forEach(function (notify) {
    notify(activeTheme)
  })
}
