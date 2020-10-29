module.exports = {
  tree: [
    [
      "CreateUser",
      [
        ["Email", []],
        [
          "AccessLevel",
          [
            ["User", ""],
            ["Librarian", ""],
            ["Admin", ""],
          ],
        ],
        ["Submit", []],
      ],
    ],
    [
      "Books",
      [
        ["Borrow", []],
        ["EndBorrow", []],
      ],
    ],
    [
      "AppBar",
      [
        ["Books", []],
        ["CreateUser", []],
        ["CreateBook", []],
        ["Login", []],
        ["Logout", []],
      ],
    ],
    [
      "Login",
      [
        ["Email", []],
        ["Submit", []],
      ],
    ],
    [
      "CreateBook",
      [
        ["Title", []],
        ["Submit", []],
      ],
    ],
  ],
}
