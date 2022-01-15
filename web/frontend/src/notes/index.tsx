import { NoteAdd } from "@mui/icons-material";
import { SpeedDial, SpeedDialAction, SpeedDialIcon } from "@mui/material";
import { FC } from "react";
import { Routes, Route, useNavigate } from "react-router-dom";

import { Create } from "./create";
import { Detail } from "./detail";
import { List } from "./list";

export const Notes: FC = () => {
  const navigate = useNavigate();
  return (
    <>
      <Routes>
        <Route path="create" element={<Create />} />
        <Route path=":id" element={<Detail />} />
        <Route path="/" element={<List />} />
      </Routes>
      <SpeedDial
        icon={<SpeedDialIcon />}
        ariaLabel="note actions"
        sx={{ position: `absolute`, bottom: 16, right: 16 }}
      >
        <SpeedDialAction
          icon={<NoteAdd />}
          tooltipTitle="Create"
          onClick={() => {
            navigate(`create`);
          }}
        />
      </SpeedDial>
    </>
  );
};
