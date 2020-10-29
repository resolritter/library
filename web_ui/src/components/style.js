import { css } from "linaria"
import { cssVariableValueOf } from "src/utils"

export const themedButton = css`
  background-color: ${cssVariableValueOf("backgroundColor")};
  color: ${cssVariableValueOf("color")};
  padding: 16px;
  font-weight: bold;
`
