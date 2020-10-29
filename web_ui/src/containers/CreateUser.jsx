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

import LoadingSubmitButton from "src/components/LoadingSubmitButton"
import { ButtonRow, Column, ColumnTitle } from "src/components/SharedForLogin"
import { routes, userUIAccessLevels } from "src/constants"
import { FullContentSpaceLayoutCentered } from "src/containers/FullContentSpaceLayout"
import { createUser } from "src/requests/user"
import { history } from "src/setup"
import { handleWithSnackbar } from "src/utils"

export function CreateUser() {
  const { enqueueSnackbar } = useSnackbar()
  const [email, setEmail] = React.useState("user@user.com")
  const [accessLevel, setAccessLevel] = React.useState("")
  const [isLoading, setIsLoading] = React.useState(false)
  const user = useSelector(function ({ user: { profile } }) {
    return profile
  })
  const errorToSnackbar = React.useMemo(
    function () {
      return handleWithSnackbar(enqueueSnackbar)
    },
    [enqueueSnackbar],
  )
  const shouldSetAsCurrent = !user

  return (
    <FullContentSpaceLayoutCentered>
      <Container maxWidth="sm">
        <Card>
          <Column>
            <ColumnTitle variant="h4">Create User</ColumnTitle>
            <form
              onSubmit={function (ev) {
                ev.preventDefault()
                errorToSnackbar(
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
                  required
                />
              </FormControl>
              <FormControl fullWidth>
                <InputLabel id="access_level_label">Access level</InputLabel>
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
                  <MenuItem value={userUIAccessLevels.admin}>Admin</MenuItem>
                </Select>
              </FormControl>
              <ButtonRow fullWidth>
                <LoadingSubmitButton {...{ isLoading }} />
              </ButtonRow>
            </form>
          </Column>
        </Card>
      </Container>
    </FullContentSpaceLayoutCentered>
  )
}

export default CreateUser
