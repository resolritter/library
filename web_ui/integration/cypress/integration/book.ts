import label from "../../../src/labels.json"

describe("Tests related to books", () => {
  it("full flow", () => {
    cy.visit(Cypress.env("UIAddr"))
    cy.get(`[aria-label='${label.AppBar.CreateUser.id}']`).click()
    cy.focused().type(`test${Cypress.env("ID")}${Date.now()}@user.com\n`)
  })
})
