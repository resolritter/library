import React from "react"
import AppBar from "@material-ui/core/AppBar"
import Toolbar from "@material-ui/core/Toolbar"
import Button from "@material-ui/core/Button"
import Box from "@material-ui/core/Box"
import { withStyles } from "@material-ui/core/Button"
import { history } from "src/setup"
import { routes } from "src/constants"

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
        </Box>
      </Toolbar>
    </AppBar>
  )
}

export default MainAppBar
