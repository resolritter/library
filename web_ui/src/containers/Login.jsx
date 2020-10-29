import React from "react"
import {
  Card,
  Container,
  FormControl,
  Input,
  InputLabel,
} from "@material-ui/core"
import { useSnackbar } from "notistack"

import LoadingSubmitButton from "src/components/LoadingSubmitButton"
import { ButtonRow, Column, ColumnTitle } from "src/components/SharedForLogin"
import { routes } from "src/constants"
import { FullContentSpaceLayoutCentered } from "src/containers/FullContentSpaceLayout"
import { login } from "src/requests/user"
import { history } from "src/setup"

export function Login() {
  const { enqueueSnackbar } = useSnackbar()
  const [email, setEmail] = React.useState("")
  const [isLoading, setIsLoading] = React.useState(false)

  return (
    <FullContentSpaceLayoutCentered>
      <Container maxWidth="sm">
        <Card>
          <Column>
            <ColumnTitle variant="h4">Login</ColumnTitle>
            <form
              onSubmit={async function (ev) {
                ev.preventDefault()
                setIsLoading(true)
                const result = await login({ email })
                if (result instanceof Error) {
                  enqueueSnackbar(result.message, { variant: "error" })
                  setIsLoading(false)
                } else {
                  history.push(routes.home())
                }
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
