import React from "react"
import { CircularProgress, Button, Box, withStyles } from "@material-ui/core"

export const ButtonProgress = withStyles({
  root: {
    position: "absolute",
  },
})(CircularProgress)

export function LoadingSubmitButton({ isLoading }) {
  return (
    <Box
      position="relative"
      display="flex"
      alignItems="center"
      justifyContent="center"
    >
      <Button
        type="submit"
        variant="contained"
        color="primary"
        disabled={isLoading}
      >
        <span style={{ opacity: isLoading ? 0 : 1 }}>Submit</span>
      </Button>
      {isLoading && <ButtonProgress size={20} />}
    </Box>
  )
}

export default LoadingSubmitButton
