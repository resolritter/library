import React from "react"
import AppBar from "@material-ui/core/AppBar"
import Box from "@material-ui/core/Box"
import Button from "@material-ui/core/Button"
import Toolbar from "@material-ui/core/Toolbar"
import { useDispatch, useSelector } from "react-redux"

import { routes, userAPIAccessLevels } from "src/constants"
import label from "src/labels.json"
import { history } from "src/setup"
import userStore from "src/store/user"
import { flexCenteredRow } from "src/styles"
import { getAccessMaskDisplayName } from "src/utils"

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
            aria-label={label.AppBar.Books.id}
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
                  aria-label={label.AppBar.CreateBook.id}
                >
                  Create Book
                </Button>
              </>
            )}
          {(!user || user.access_mask === userAPIAccessLevels.admin) && (
            <>
              <ButtonGap />
              <ButtonGap separator={"•"} />
              <ButtonGap />
              <Button
                aria-label={label.AppBar.CreateUser.id}
                onClick={function () {
                  history.push(routes.createUser())
                }}
                color="inherit"
              >
                Create User
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
              aria-label={label.AppBar.Login.id}
            >
              Login
            </Button>
          )}
          {user && (
            <>
              <span>
                User: {user.email} (access:{" "}
                {getAccessMaskDisplayName(user.access_mask)})
              </span>
              <ButtonGap />
              <Button
                onClick={function () {
                  dispatch(userStore.actions.setUser())
                }}
                variant="contained"
                aria-label={label.AppBar.Logout.id}
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
