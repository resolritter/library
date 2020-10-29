import { createSlice } from "@reduxjs/toolkit"

export const initialState = { items: [] }

export default createSlice({
  name: "book",
  initialState,
  reducers: {
    addBooks: function (state, { payload: books }) {
      state.items = state.items.concat(books)
    },
  },
})
