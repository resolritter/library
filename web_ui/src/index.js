import { ConnectedRouter } from "connected-react-router"
import { SnackbarProvider } from "notistack"
import * as React from "react"
import * as ReactDOM from "react-dom"
import { Provider } from "react-redux"

import App from "./App"
import { history, store } from "./setup"
import { initialTheme, setTheme } from "./theme"

setTheme(initialTheme)

ReactDOM.render(
  <SnackbarProvider>
    <Provider store={store}>
      <ConnectedRouter history={history}>
        <App />
      </ConnectedRouter>
    </Provider>
  </SnackbarProvider>,
  document.getElementById("app"),
)
