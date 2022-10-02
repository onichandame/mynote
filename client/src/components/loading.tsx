import { Box, CircularProgress, Modal } from "@mui/material"

export function Loading() {
  return (
    <Modal open={true}>
      <Box
        sx={{
          position: `absolute`,
          top: `50%`,
          left: `50%`,
          transform: `translate(-50%,-50%)`,
        }}
      >
        <CircularProgress />
      </Box>
    </Modal>
  )
}
