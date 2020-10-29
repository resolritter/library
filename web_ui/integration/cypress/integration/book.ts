import label from "../../../src/labels.json"

describe("example", () => {
  it("full flow", () => {
    cy.visit("http://localhost:3000")
    cy.get(`[aria-label='${label.AppBar.CreateUser.id}']`).click()
    cy.focused().type("test@user.com\n")
  })
})
