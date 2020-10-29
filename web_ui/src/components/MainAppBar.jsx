import AppBar from "@material-ui/core/AppBar"
import Box from "@material-ui/core/Box"
import Button from "@material-ui/core/Button"
import Toolbar from "@material-ui/core/Toolbar"
import React from "react"
import { routes } from "src/constants"
import { history } from "src/setup"

export const MainAppBar = function () {
  return (
    <AppBar position="static">
      <Toolbar variant="dense">
        <Box display="flex" justifyContent="flex-end" flex={1}>
          <Button
            onClick={function () {
              history.push(routes.login())
            }}
            color="inherit"
          >
            Login
          </Button>
          <Button
            onClick={function () {
              history.push(routes.createUser())
            }}
            color="inherit"
          >
            Create user
          </Button>
        </Box>
      </Toolbar>
    </AppBar>
  )
}

export default MainAppBar
