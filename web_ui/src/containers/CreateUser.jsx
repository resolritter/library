import React from "react"
import {
  Card,
  CardContent,
  Container,
  FormControl,
  Input,
  InputLabel,
  MenuItem,
  Select,
  Typography,
  withStyles,
} from "@material-ui/core"
import { useSnackbar } from "notistack"

import LoadingSubmitButton from "src/components/LoadingSubmitButton"
import { routes, userUIAccessLevels } from "src/constants"
import { FullContentSpaceLayoutCentered } from "src/containers/FullContentSpaceLayout"
import { createUser } from "src/requests/user"
import { history } from "src/setup"
import { flexCenteredColumn } from "src/styles"

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
  const { enqueueSnackbar } = useSnackbar()
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
              onSubmit={async function (ev) {
                ev.preventDefault()
                setIsLoading(true)
                const result = await createUser({ email, accessLevel })
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
