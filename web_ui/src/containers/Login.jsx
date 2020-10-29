import React from "react"
import {
  Card,
  CardContent,
  Container,
  FormControl,
  Input,
  InputLabel,
  Typography,
  withStyles,
} from "@material-ui/core"
import { useSnackbar } from "notistack"

import LoadingSubmitButton from "src/components/LoadingSubmitButton"
import { routes } from "src/constants"
import { FullContentSpaceLayoutCentered } from "src/containers/FullContentSpaceLayout"
import { login } from "src/requests/user"
import { history } from "src/setup"
import { flexCenteredColumn } from "src/styles"

const LoginColumn = withStyles({
  root: flexCenteredColumn,
})(CardContent)

const LoginColumnTitle = withStyles({
  root: {
    marginBottom: "0.5em",
  },
})(Typography)

const LoginButtonRow = withStyles({
  root: {
    marginTop: "1.2em",
  },
})(FormControl)

export function Login() {
  const { enqueueSnackbar } = useSnackbar()
  const [email, setEmail] = React.useState("")
  const [isLoading, setIsLoading] = React.useState(false)

  return (
    <FullContentSpaceLayoutCentered>
      <Container maxWidth="sm">
        <Card>
          <LoginColumn>
            <LoginColumnTitle variant="h4">Login</LoginColumnTitle>
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
              <LoginButtonRow fullWidth>
                <LoadingSubmitButton {...{ isLoading }} />
              </LoginButtonRow>
            </form>
          </LoginColumn>
        </Card>
      </Container>
    </FullContentSpaceLayoutCentered>
  )
}

export default Login
