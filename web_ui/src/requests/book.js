import { StatusCodes } from "http-status-codes"

import { getCors, handleErrorResponse } from "."

import { apiEndpoints } from "src/constants"
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
