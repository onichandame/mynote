import { ContentCopy } from "@mui/icons-material";
import { IconButton } from "@mui/material";
import { FC } from "react";

/** A button which writes text to clipboard */
export const Clipboard: FC<{ value: string }> = ({ value }) => {
  return (
    <IconButton
      onClick={async (e) => {
        e.stopPropagation();
        navigator.clipboard.writeText(value);
      }}
    >
      <ContentCopy />
    </IconButton>
  );
};
