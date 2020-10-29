import * as React from "react"
import { ConnectedRouter } from "connected-react-router"
import { SnackbarProvider } from "notistack"
import * as ReactDOM from "react-dom"
import { Provider } from "react-redux"

import App from "./App"
import { history, store } from "./setup"

window.notistackRef = React.createRef()

ReactDOM.render(
  <SnackbarProvider ref={window.notistackRef}>
    <Provider store={store}>
      <ConnectedRouter history={history}>
        <App />
      </ConnectedRouter>
    </Provider>
  </SnackbarProvider>,
  document.getElementById("app"),
)
