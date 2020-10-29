import { createSlice } from "@reduxjs/toolkit"

export const initialState = { items: [] }

export default createSlice({
  name: "book",
  initialState,
  reducers: {
    addMoreBooks: function (state, { payload: books }) {
      state.items.concat(books)
    },
  },
})
