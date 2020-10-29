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
import { routes, userAPIAccessLevels } from "src/constants"
import { FullContentSpaceLayoutCentered } from "src/containers/FullContentSpaceLayout"
import { createBook } from "src/requests/book"
import { history } from "src/setup"
import { promiseToSnackbar } from "src/utils"

export function CreateBook() {
  const { enqueueSnackbar } = useSnackbar()
  const [title, setTitle] = React.useState("")
  const user = useSelector(function ({ user: { profile } }) {
    return profile
  })
  const handleWithSnackbar = React.useMemo(
    function () {
      return promiseToSnackbar(enqueueSnackbar)
    },
    [enqueueSnackbar],
  )
  const isAuthorized =
    user &&
    (user.access_mask & userAPIAccessLevels.librarian) ===
      userAPIAccessLevels.librarian
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
            <ColumnTitle variant="h4">Create Book</ColumnTitle>
            <form
              onSubmit={function (ev) {
                ev.preventDefault()
                handleWithSnackbar(createBook(user, { title }), function () {
                  enqueueSnackbar("Done!", {
                    variant: "success",
                  })
                  setTitle("")
                })
              }}
            >
              <FormControl fullWidth>
                <InputLabel htmlFor="title">Title</InputLabel>
                <Input
                  type="title"
                  name="title"
                  value={title}
                  onChange={function (ev) {
                    setTitle(ev.target.value)
                  }}
                  required
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

export default CreateBook
