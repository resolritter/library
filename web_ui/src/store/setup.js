import { configureStore } from "@reduxjs/toolkit"
import { routerMiddleware } from "connected-react-router"
import { connectRouter } from "connected-react-router"
import { createBrowserHistory } from "history"

import book from "./book"
import user from "./user"

export default function (preloadedState) {
  const history = createBrowserHistory()
  const middleware = [routerMiddleware(history)]
  const enhancers = []

  const store = configureStore({
    reducer: {
      user: user.reducer,
      book: book.reducer,
      router: connectRouter(history),
    },
    middleware,
    preloadedState,
    enhancers,
  })

  return { store, history }
}
