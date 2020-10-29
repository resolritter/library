import { StatusCodes } from "http-status-codes"

import { getCors, handleErrorResponse } from "."

import { apiEndpoints, week } from "src/constants"
import { store } from "src/setup"
import booksStore from "src/store/book"

export const loadBooks = async function () {
  const response = await fetch(apiEndpoints.books(), {
    mode: getCors(),
  })

  if (response.status === StatusCodes.OK) {
    store.dispatch(booksStore.actions.addBooks(await response.json()))
    return { status: "ok" }
  } else {
    return await handleErrorResponse(response)
  }
}

export const borrowBook = async function ({ access_token, email }, { title }) {
  const response = await fetch(apiEndpoints.borrowBook({ title }), {
    method: "POST",
    mode: getCors(),
    headers: {
      "Content-Type": "application/json",
      "X-Auth": access_token,
    },
    body: JSON.stringify({
      title,
      borrow_id: email,
      borrow_length: week,
    }),
  })

  if (response.status !== StatusCodes.OK) {
    return await handleErrorResponse(response)
  }
}

export const endBookBorrow = async function (
  { email, access_token },
  { title },
) {
  const response = await fetch(
    apiEndpoints.borrowBook({ title, borrowId: email }),
    {
      method: "DELETE",
      mode: getCors(),
      headers: {
        "Content-Type": "application/json",
        "X-Auth": access_token,
      },
      body: JSON.stringify({
        title,
      }),
    },
  )

  if (response.status !== StatusCodes.OK) {
    return await handleErrorResponse(response)
  }
}
