import label from "../../../src/labels.json"
import {
  click,
  getUniqueString,
  getUniqueEmail,
  waitForDoneNotification,
} from "../utils"

describe("Borrowing-related tests", () => {
  it("Borrow a book, then end its borrow", () => {
    cy.visit(Cypress.env("UIAddr"))

    // Log in with the administrator to create a librarian
    click(cy, label.AppBar.Login.id)
    cy.focused().type("admin@admin.com\n")
    click(cy, label.AppBar.CreateUser.id)
    const librarian = getUniqueEmail()
    cy.focused().type(librarian)
    click(cy, label.CreateUser.AccessLevel.id)
    click(cy, label.CreateUser.AccessLevel.Librarian.id)
    waitForDoneNotification(cy, label.CreateUser.Submit.id)
    click(cy, label.AppBar.Logout.id)

    // Now log in with the librarian
    click(cy, label.AppBar.Login.id)
    cy.focused().type(librarian)
    click(cy, label.Login.Submit.id)

    // Create the book
    click(cy, label.AppBar.CreateBook.id)
    const book = getUniqueString()
    cy.focused().type(book)
    click(cy, label.CreateBook.Submit.id)
    waitForDoneNotification(cy, label.CreateBook.Submit.id)

    // Log out
    click(cy, label.AppBar.Logout.id)

    // Create a normal user
    click(cy, label.AppBar.CreateUser.id)
    const user = getUniqueEmail()
    cy.focused().type(user)
    click(cy, label.CreateUser.Submit.id)

    // Go to the books route
    click(cy, label.AppBar.Books.id)

    // Borrow the book which was just created
    click(cy, `${label.Books.Borrow.id}__${book}`)

    // Log out
    click(cy, label.AppBar.Logout.id)

    // Switch back to the librarian
    click(cy, label.AppBar.Login.id)
    cy.focused().type(librarian)
    click(cy, label.Login.Submit.id)

    // Cancel the borrow
    click(cy, `${label.Books.EndBorrow.id}__${book}`)
  })
})
