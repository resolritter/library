import * as React from "react"
import { Route, Switch } from "react-router"

import MainAppBar from "src/components/MainAppBar"
import { routes } from "src/constants"
import Books from "src/containers/Books"
import CreateBook from "src/containers/CreateBook"
import CreateUser from "src/containers/CreateUser"
import FullPageLayout from "src/containers/FullPageLayout"
import Login from "src/containers/Login"

export const ConnectedApp = function () {
  return (
    <FullPageLayout>
      <MainAppBar />
      <Switch>
        <Route exact path={routes.home()} component={Books} />
        <Route exact path={routes.createBook()} component={CreateBook} />
        <Route exact path={routes.login()} component={Login} />
        <Route exact path={routes.createUser()} component={CreateUser} />
        <Route
          component={function NotFound() {
            return <div>404</div>
          }}
        />
      </Switch>
    </FullPageLayout>
  )
}

export default ConnectedApp
