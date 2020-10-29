import { styled } from "linaria/react"

import { flexCenteredColumn } from "src/styles"

// 48px is the minimum height for Material UI's app bar
const defaultStyle = `
height: calc(100vh - 48px);
width: 100vw;
`
export const FullContentSpaceLayout = styled.div`
  ${defaultStyle}
`
export const FullContentSpaceLayoutCentered = styled.div`
  ${defaultStyle}
  ${flexCenteredColumn}
`
