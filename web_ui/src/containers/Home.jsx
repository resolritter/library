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

import { borrowBook, loadBooks } from "src/requests/book"
import { loadingStates } from "src/utils"

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
              <TableHeaderCell></TableHeaderCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {books.map(function ({ title, borrow_until }, i) {
              const canBeBorrowed = !borrow_until
              console.log(books[i], user)
              return (
                <TableRow hover key={i}>
                  <TableCell>{title}</TableCell>
                  <TableCell>
                    {canBeBorrowed ? "Can be borrowed" : borrow_until}
                  </TableCell>
                  <TableCell>
                    {user && canBeBorrowed && (
                      <div>
                        <Button
                          onClick={async function () {
                            borrowBook({ title, email: user.email })
                              .then(function (result) {
                                if (result instanceof Error) {
                                  enqueueSnackbar(result.message, {
                                    variant: "error",
                                  })
                                } else {
                                  reloadBooks()
                                }
                              })
                              .catch(function (err) {
                                enqueueSnackbar(err.message, {
                                  variant: "error",
                                })
                              })
                          }}
                        >
                          Borrow
                        </Button>
                      </div>
                    )}
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
