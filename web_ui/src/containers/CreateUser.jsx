import React from "react"
import { useDispatch, useSelector } from "react-redux"
import { fpGet } from "lodash/fp"
import {
  withStyles,
  FormControl,
  InputLabel,
  Input,
  FormHelperText,
  Button,
  Card,
  CardContent,
  Container,
  Box,
  Typography,
  Select,
  MenuItem,
} from "@material-ui/core"

import { FullContentSpaceLayoutCentered } from "src/containers/FullContentSpaceLayout"
import { flexCenteredColumn } from "src/styles"
import LoadingSubmitButton from "src/components/LoadingSubmitButton"

const CreateUserColumn = withStyles({
  root: flexCenteredColumn,
})(CardContent)

const CreateUserColumnTitle = withStyles({
  root: {
    marginBottom: "0.5em",
  },
})(Typography)

const CreateUserButtonRow = withStyles({
  root: {
    marginTop: "1.2em",
  },
})(FormControl)

export function CreateUser() {
  const createUserForm = React.useRef()
  const [email, setEmail] = React.useState("user@user.com")
  const [accessLevel, setAccessLevel] = React.useState("")
  const [isLoading, setIsLoading] = React.useState(false)

  return (
    <FullContentSpaceLayoutCentered>
      <Container maxWidth="sm">
        <Card>
          <CreateUserColumn>
            <CreateUserColumnTitle variant="h4">
              Create User
            </CreateUserColumnTitle>
            <form
              ref={createUserForm}
              onSubmit={function (ev) {
                ev.preventDefault()
                setIsLoading(true)
                console.log({ email, accessLevel })
              }}
            >
              <FormControl fullWidth>
                <InputLabel htmlFor="email">Email address</InputLabel>
                <Input
                  type="email"
                  name="email"
                  id="email"
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
                  id="access_level"
                >
                  <MenuItem value={""}>None</MenuItem>
                  <MenuItem value={"librarian"}>Librarian</MenuItem>
                  <MenuItem value={"admin"}>Admin</MenuItem>
                </Select>
              </FormControl>
              <CreateUserButtonRow fullWidth>
                <LoadingSubmitButton {...{ isLoading }} />
              </CreateUserButtonRow>
            </form>
          </CreateUserColumn>
        </Card>
      </Container>
    </FullContentSpaceLayoutCentered>
  )
}

export default CreateUser
