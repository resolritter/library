export const getUniqueString = function (suffix = "") {
  return `${Cypress.env("ID")}${Date.now()}${suffix}`
}

export const getUniqueEmail = function () {
  return getUniqueString("@user.com")
}

export const waitForDoneNotification = function (
  cy: Cypress.cy,
  submitLabel: string,
) {
  return cy.window().then(function (win) {
    cy.wrap(null, { timeout: 10000 }).then(() => {
      return new Cypress.Promise((resolve, reject) => {
        const body = win.document.body
        const observer = new MutationObserver(function () {
          for (const notification of Array.from(
            body.querySelectorAll('[class^="SnackbarItem-message"]'),
          )) {
            if (notification.innerHTML.indexOf("Done")) {
              observer.disconnect()
              ;(win as any).notistackRef.current.closeSnackbar()
              resolve()
            }
          }
        })
        observer.observe(body, { subtree: true, childList: true })
        const submit = body.querySelector(
          `[aria-label='${submitLabel}']`,
        ) as HTMLElement
        submit.click()
      })
    })
  })
}

export const click = function (cy: Cypress.cy, label: string) {
  cy.get(`[aria-label='${label}']`).click()
}
