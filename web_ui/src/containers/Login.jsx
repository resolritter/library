import { withStyles } from "@material-ui/core"
import Button from "@material-ui/core/Button"
import Card from "@material-ui/core/Card"
import CardContent from "@material-ui/core/CardContent"
import Container from "@material-ui/core/Container"
import FormControl from "@material-ui/core/FormControl"
import Input from "@material-ui/core/Input"
import InputLabel from "@material-ui/core/InputLabel"
import Typography from "@material-ui/core/Typography"
import React from "react"
import { FullContentSpaceLayoutCentered } from "src/containers/FullContentSpaceLayout"
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
  return (
    <FullContentSpaceLayoutCentered>
      <Container maxWidth="sm">
        <Card>
          <LoginColumn>
            <LoginColumnTitle variant="h4">Login</LoginColumnTitle>
            <form
              onSubmit={function (ev) {
                ev.preventDefault()
              }}
            >
              <FormControl fullWidth>
                <InputLabel htmlFor="email">Email address</InputLabel>
                <Input
                  type="email"
                  name="email"
                  id="email"
                  aria-describedby="my-helper-text"
                />
              </FormControl>
              <LoginButtonRow fullWidth>
                <Button type="submit" variant="contained" color="primary">
                  Login
                </Button>
              </LoginButtonRow>
            </form>
          </LoginColumn>
        </Card>
      </Container>
    </FullContentSpaceLayoutCentered>
  )
}

export default Login
