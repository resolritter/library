import React from "react"
import {
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

import { loadBooks } from "src/requests/book"
import { loadingStates } from "src/utils"

const TableHeaderCell = withStyles({
  root: {
    fontWeight: "bold",
    fontSize: "1.2rem",
  },
})(TableCell)

export function Home() {
  const hasLoaded = React.useRef(loadingStates.notStarted)
  const [isLoaded, setIsLoaded] = React.useState(false)
  const { enqueueSnackbar } = useSnackbar()

  const books = useSelector(function ({ book: { items } }) {
    return items
  })

  React.useLayoutEffect(
    function () {
      if (hasLoaded.current !== loadingStates.notStarted) {
        return
      }
      hasLoaded.current = loadingStates.loading
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
    [hasLoaded],
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
            </TableRow>
          </TableHead>
          <TableBody>
            {books.map(function ({ title, lease_until }, i) {
              return (
                <TableRow hover key={i}>
                  <TableCell>{title}</TableCell>
                  <TableCell>{lease_until ?? "Borrowable"}</TableCell>
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
