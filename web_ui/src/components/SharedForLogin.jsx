import {
  CardContent,
  FormControl,
  Typography,
  withStyles,
} from "@material-ui/core"

import { flexCenteredColumn } from "src/styles"

export const Column = withStyles({
  root: flexCenteredColumn,
})(CardContent)

export const ColumnTitle = withStyles({
  root: {
    marginBottom: "0.5em",
  },
})(Typography)

export const ButtonRow = withStyles({
  root: {
    marginTop: "1.2em",
  },
})(FormControl)
