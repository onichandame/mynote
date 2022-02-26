import { FC } from "react";
import { Routes, Route } from "react-router-dom";

import { Create } from "./create";
import { Note } from "./note";
import { List } from "./list";

export const Notes: FC = () => {
  return (
    <>
      <Routes>
        <Route path="create" element={<Create />} />
        <Route path=":id/*" element={<Note />} />
        <Route path="/" element={<List />} />
      </Routes>
    </>
  );
};
