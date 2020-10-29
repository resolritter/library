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

import { ButtonRow, Column, ColumnTitle } from "src/components/Form"
import LoadingSubmitButton from "src/components/LoadingSubmitButton"
import { routes } from "src/constants"
import { FullContentSpaceLayoutCentered } from "src/containers/FullContentSpaceLayout"
import { login } from "src/requests/user"
import { history } from "src/setup"
import { promiseToSnackbar } from "src/utils"

export function Login() {
  const { enqueueSnackbar } = useSnackbar()
  const [email, setEmail] = React.useState("")
  const user = useSelector(function ({ user: { profile } }) {
    return profile
  })
  const handleWithSnackbar = React.useMemo(
    function () {
      return promiseToSnackbar(enqueueSnackbar)
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
                handleWithSnackbar(login({ email }), function () {
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
                <LoadingSubmitButton isLoading={false} />
              </ButtonRow>
            </form>
          </Column>
        </Card>
      </Container>
    </FullContentSpaceLayoutCentered>
  )
}

export default Login
