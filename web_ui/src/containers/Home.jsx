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
  withStyles,
} from "@material-ui/core"
import sortBy from "lodash-es/sortBy"
import { useSnackbar } from "notistack"
import { useSelector } from "react-redux"

import { userAPIAccessLevels } from "src/constants"
import { borrowBook, endBookBorrow, loadBooks } from "src/requests/book"
import { loadingStates, promiseToSnackbar } from "src/utils"

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
    return sortBy(items, "title")
  })
  const handleWithSnackbar = React.useMemo(
    function () {
      return promiseToSnackbar(enqueueSnackbar)
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
    <TableContainer component={Paper}>
      <Table size="small">
        <TableHead>
          <TableRow>
            <TableHeaderCell>Title</TableHeaderCell>
            <TableHeaderCell>Status</TableHeaderCell>
            <TableHeaderCell></TableHeaderCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {books.map(function ({ title, borrow_until }, i) {
            const canBeBorrowed = !borrow_until
            const borrowedUntilDate = new Date(borrow_until * 1000)
            const isBorrowBookShown = user && canBeBorrowed
            const isCancelBorrowBookShown =
              !isBorrowBookShown &&
              user &&
              (userAPIAccessLevels.librarian & user.access_mask) ==
                userAPIAccessLevels.librarian

            const borrowButton = (
              <Button
                key="borrowButton"
                variant="contained"
                color="primary"
                onClick={function () {
                  handleWithSnackbar(borrowBook(user, { title }), reloadBooks)
                }}
                style={
                  isBorrowBookShown
                    ? undefined
                    : { opacity: 0, pointerEvents: "none", cursor: "none" }
                }
              >
                Borrow
              </Button>
            )
            const cancelBorrowButton = (
              <Button
                key="cancelBorrowButton"
                variant="contained"
                color="secondary"
                onClick={function () {
                  handleWithSnackbar(
                    endBookBorrow(user, { title }),
                    reloadBooks,
                  )
                }}
                style={
                  isCancelBorrowBookShown
                    ? undefined
                    : { opacity: 0, pointerEvents: "none", cursor: "none" }
                }
              >
                Cancel Borrow
              </Button>
            )
            const buttons = isCancelBorrowBookShown
              ? [cancelBorrowButton, borrowButton]
              : [borrowButton, cancelBorrowButton]

            return (
              <TableRow hover key={i}>
                <TableCell>{title}</TableCell>
                <TableCell style={{ width: 1, whiteSpace: "nowrap" }}>
                  {canBeBorrowed ? (
                    "Available"
                  ) : (
                    <span>
                      <b>Borrowed</b> until{" "}
                      {borrowedUntilDate.toString().slice(0, 16)}
                    </span>
                  )}
                </TableCell>
                <TableCell style={{ width: 1, whiteSpace: "nowrap" }}>
                  {buttons}
                </TableCell>
              </TableRow>
            )
          })}
        </TableBody>
      </Table>
    </TableContainer>
  )
}

export default Home
