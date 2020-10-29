import React from "react"
import {
  Card,
  Container,
  FormControl,
  Input,
  InputLabel,
} from "@material-ui/core"
import { useSnackbar } from "notistack"
import { useSelector } from "react-redux"

import LoadingSubmitButton from "src/components/LoadingSubmitButton"
import { ButtonRow, Column, ColumnTitle } from "src/components/SharedForLogin"
import { routes } from "src/constants"
import { FullContentSpaceLayoutCentered } from "src/containers/FullContentSpaceLayout"
import { login } from "src/requests/user"
import { history } from "src/setup"
import { handleWithSnackbar } from "src/utils"

export function Login() {
  const { enqueueSnackbar } = useSnackbar()
  const [email, setEmail] = React.useState("")
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
  React.useLayoutEffect(
    function () {
      if (user) {
        history.push(routes.home())
      }
    },
    [user],
  )

  return (
    <FullContentSpaceLayoutCentered>
      <Container maxWidth="sm">
        <Card>
          <Column>
            <ColumnTitle variant="h4">Login</ColumnTitle>
            <form
              onSubmit={function (ev) {
                ev.preventDefault()
                errorToSnackbar(login({ email }), function () {
                  history.push(routes.home())
                })
              }}
            >
              <FormControl fullWidth>
                <InputLabel htmlFor="email">Email address</InputLabel>
                <Input
                  type="email"
                  name="email"
                  id="email"
                  aria-describedby="my-helper-text"
                  onChange={function (ev) {
                    setEmail(ev.target.value)
                  }}
                />
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

export default Login
