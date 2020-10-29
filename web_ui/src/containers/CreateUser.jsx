import React from "react"
import {
  Card,
  Container,
  FormControl,
  Input,
  InputLabel,
  MenuItem,
  Select,
} from "@material-ui/core"
import { useSnackbar } from "notistack"
import { useSelector } from "react-redux"

import { ButtonRow, Column, ColumnTitle } from "src/components/Form"
import LoadingSubmitButton from "src/components/LoadingSubmitButton"
import { routes, userAPIAccessLevels, userUIAccessLevels } from "src/constants"
import { FullContentSpaceLayoutCentered } from "src/containers/FullContentSpaceLayout"
import label from "src/labels.json"
import { createUser } from "src/requests/user"
import { history } from "src/setup"
import { promiseToSnackbar } from "src/utils"

export function CreateUser() {
  const { enqueueSnackbar } = useSnackbar()
  const [email, setEmail] = React.useState("")
  const [accessLevel, setAccessLevel] = React.useState("")
  const user = useSelector(function ({ user: { profile } }) {
    return profile
  })
  const handleWithSnackbar = React.useMemo(
    function () {
      return promiseToSnackbar(enqueueSnackbar)
    },
    [enqueueSnackbar],
  )
  const shouldSetAsCurrent = !user
  const isAuthorized =
    !user ||
    (user &&
      (user.access_mask & userAPIAccessLevels.admin) ===
        userAPIAccessLevels.admin)
  React.useLayoutEffect(
    function () {
      if (!isAuthorized) {
        history.push(routes.home())
      }
    },
    [isAuthorized],
  )
  if (!isAuthorized) {
    return null
  }

  return (
    <FullContentSpaceLayoutCentered>
      <Container maxWidth="sm">
        <Card>
          <Column>
            <ColumnTitle variant="h4">Create User</ColumnTitle>
            <form
              onSubmit={function (ev) {
                ev.preventDefault()
                handleWithSnackbar(
                  createUser({ email, accessLevel, shouldSetAsCurrent }),
                  function () {
                    if (shouldSetAsCurrent) {
                      history.push(routes.home())
                    } else {
                      enqueueSnackbar("Done!", {
                        variant: "success",
                      })
                    }
                  },
                )
              }}
            >
              <FormControl fullWidth>
                <InputLabel htmlFor="email">Email address</InputLabel>
                <Input
                  type="email"
                  name="email"
                  value={email}
                  onChange={function (ev) {
                    setEmail(ev.target.value)
                  }}
                  placeholder={"user@mail.com"}
                  aria-label={label.CreateUser.Email.id}
                  autoFocus
                  required
                />
              </FormControl>
              {user?.access_mask === userAPIAccessLevels.admin && (
                <>
                  <FormControl key="access_level" fullWidth>
                    <InputLabel id="access_level_label">
                      Access Level
                    </InputLabel>
                    <Select
                      value={accessLevel}
                      onChange={function (ev) {
                        setAccessLevel(ev.target.value)
                      }}
                      labelId="access_level_label"
                    >
                      <MenuItem value={""}>None</MenuItem>
                      <MenuItem value={userUIAccessLevels.librarian}>
                        Librarian
                      </MenuItem>
                      <MenuItem value={userUIAccessLevels.admin}>
                        Admin
                      </MenuItem>
                    </Select>
                  </FormControl>
                </>
              )}
              <ButtonRow fullWidth>
                <LoadingSubmitButton isLoading={false} />
              </ButtonRow>
            </form>
          </Column>
        </Card>
      </Container>
    </FullContentSpaceLayoutCentered>
  )
}

export default CreateUser
