import AppBar from "@material-ui/core/AppBar"
import Box from "@material-ui/core/Box"
import Button from "@material-ui/core/Button"
import Toolbar from "@material-ui/core/Toolbar"
import React from "react"
import { useDispatch, useSelector } from "react-redux"
import { routes } from "src/constants"
import { history } from "src/setup"
import userStore from "src/store/user"

export const MainAppBar = function () {
  const user = useSelector(function ({ user }) {
    return user.profile
  })
  const dispatch = useDispatch()

  return (
    <AppBar position="static">
      <Toolbar variant="dense">
        <Box
          display="flex"
          justifyContent="flex-end"
          alignItems="center"
          flex={1}
        >
          {!user && (
            <>
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
            </>
          )}
          {user && (
            <>
              <span>User: {user.email}</span>
              <Button
                onClick={function () {
                  dispatch(userStore.actions.setUser())
                }}
                color="inherit"
              >
                LOGOUT
              </Button>
            </>
          )}
        </Box>
      </Toolbar>
    </AppBar>
  )
}

export default MainAppBar
