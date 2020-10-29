import React from "react"
import {
  Button,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Typography,
  withStyles,
} from "@material-ui/core"
import { useSnackbar } from "notistack"
import { useSelector } from "react-redux"

import { userAPIAccessLevels } from "src/constants"
import { borrowBook, endBookBorrow, loadBooks } from "src/requests/book"
import { handleWithSnackbar, loadingStates } from "src/utils"

const TableHeaderCell = withStyles({
  root: {
    fontWeight: "bold",
    fontSize: "1.2rem",
  },
})(TableCell)

export function Home() {
  const { enqueueSnackbar } = useSnackbar()
  const hasLoaded = React.useRef(loadingStates.notStarted)
  const [isLoaded, setIsLoaded] = React.useState(false)
  const user = useSelector(function ({ user: { profile } }) {
    return profile
  })
  const books = useSelector(function ({ book: { items } }) {
    return items
  })
  const errorToSnackbar = React.useMemo(
    function () {
      return handleWithSnackbar(enqueueSnackbar)
    },
    [enqueueSnackbar],
  )

  const reloadBooks = React.useCallback(
    function () {
      loadBooks()
        .then(function (result) {
          if (result instanceof Error) {
            enqueueSnackbar(result.message, { variant: "error" })
          } else {
            setIsLoaded(true)
          }
        })
        .catch(function (err) {
          enqueueSnackbar(err.message, { variant: "error" })
        })
    },
    [enqueueSnackbar, setIsLoaded],
  )

  React.useLayoutEffect(
    function () {
      if (hasLoaded.current !== loadingStates.notStarted) {
        return
      }
      hasLoaded.current = loadingStates.loading
      reloadBooks()
    },
    [hasLoaded, reloadBooks],
  )

  if (!isLoaded) {
    return "Loading..."
  }

  if (!books.length) {
    return "No books to show!"
  }

  return (
    <>
      <Typography variant="h3">Book list</Typography>
      <TableContainer component={Paper}>
        <Table size="small">
          <TableHead>
            <TableRow>
              <TableHeaderCell>Title</TableHeaderCell>
              <TableHeaderCell>Availability</TableHeaderCell>
              <TableHeaderCell size="small"></TableHeaderCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {books.map(function ({ title, borrow_until }, i) {
              const canBeBorrowed = !borrow_until
              const borrowedUntilDate = new Date(borrow_until * 1000)
              return (
                <TableRow hover key={i}>
                  <TableCell>{title}</TableCell>
                  <TableCell>
                    {canBeBorrowed ? (
                      "Can be borrowed"
                    ) : (
                      <span>
                        <b>Borrowed</b> until{" "}
                        {borrowedUntilDate.toString().slice(0, 16)}
                      </span>
                    )}
                  </TableCell>
                  <TableCell size="small">
                    <Button
                      variant="contained"
                      color="primary"
                      onClick={function () {
                        errorToSnackbar(
                          borrowBook(user, { title }),
                          reloadBooks,
                        )
                      }}
                      style={{ opacity: user && canBeBorrowed ? 1 : 0 }}
                    >
                      Borrow
                    </Button>
                    <Button
                      variant="contained"
                      color="secondary"
                      onClick={function () {
                        errorToSnackbar(
                          endBookBorrow(user, { title }),
                          reloadBooks,
                        )
                      }}
                      style={{
                        opacity:
                          user &&
                          !canBeBorrowed &&
                          (userAPIAccessLevels.librarian & user.access_mask) ==
                            userAPIAccessLevels.librarian
                            ? 1
                            : 0,
                      }}
                    >
                      Cancel Borrow
                    </Button>
                  </TableCell>
                </TableRow>
              )
            })}
          </TableBody>
        </Table>
      </TableContainer>
    </>
  )
}

export default Home
