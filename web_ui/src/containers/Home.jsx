import {
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
} from "@material-ui/core"
import React from "react"
import { useSelector } from "react-redux"

export function Home() {
  const books = useSelector(function ({ book: { items } }) {
    return items
  })

  if (!books.length) {
    return "No books yet to show..."
  }

  return (
    <TableContainer component={Paper}>
      <Table>
        <TableHead>
          <TableRow>
            <TableCell>Title</TableCell>
            <TableCell>Borrowed until</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {books.map(function ({ title, lease_until }, i) {
            return (
              <TableRow key={i}>
                <TableCell>{title}</TableCell>
                <TableCell>{lease_until}</TableCell>
              </TableRow>
            )
          })}
        </TableBody>
      </Table>
    </TableContainer>
  )
}

export default Home
