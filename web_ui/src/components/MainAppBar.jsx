import React from "react"
import AppBar from "@material-ui/core/AppBar"
import Box from "@material-ui/core/Box"
import Button from "@material-ui/core/Button"
import Toolbar from "@material-ui/core/Toolbar"
import { useDispatch, useSelector } from "react-redux"

import { routes, userAPIAccessLevels } from "src/constants"
import { history } from "src/setup"
import userStore from "src/store/user"
import { flexCenteredRow } from "src/styles"

export const ButtonGap = function ({ separator = "" }) {
  return <Box width="1rem">{separator}</Box>
}

export const MainAppBar = function () {
  const user = useSelector(function ({ user }) {
    return user.profile
  })
  const dispatch = useDispatch()

  return (
    <AppBar position="static">
      <Toolbar variant="dense">
        <Box {...flexCenteredRow}>
          <Button
            onClick={function () {
              history.push(routes.home())
            }}
            color="inherit"
          >
            Books
          </Button>
          {user &&
            (user.access_mask & userAPIAccessLevels.librarian) ===
              userAPIAccessLevels.librarian && (
              <>
                <ButtonGap />
                <ButtonGap separator={"•"} />
                <ButtonGap />
                <Button
                  onClick={function () {
                    history.push(routes.createBook())
                  }}
                  color="inherit"
                >
                  Create Book
                </Button>
              </>
            )}
        </Box>
        <Box
          display="flex"
          justifyContent="flex-end"
          alignItems="center"
          flex={1}
        >
          {!user && (
            <Button
              onClick={function () {
                history.push(routes.login())
              }}
              variant="contained"
            >
              Login
            </Button>
          )}
          {user && (
            <>
              <span>User: {user.email}</span>
              <ButtonGap />
              <Button
                onClick={function () {
                  dispatch(userStore.actions.setUser())
                }}
                variant="contained"
              >
                LOGOUT
              </Button>
            </>
          )}
          {(!user || user.access_mask === userAPIAccessLevels.admin) && (
            <>
              <ButtonGap />
              <Button
                onClick={function () {
                  history.push(routes.createUser())
                }}
                variant="contained"
                color="primary"
              >
                Create user
              </Button>
            </>
          )}
        </Box>
      </Toolbar>
    </AppBar>
  )
}

export default MainAppBar
