import { FC } from "react";
import { Routes, Route } from "react-router-dom";

import { Create } from "./create";
import { Detail } from "./detail";
import { List } from "./list";

export const Notes: FC = () => {
  return (
    <>
      <Routes>
        <Route path="create" element={<Create />} />
        <Route path=":id/*" element={<Detail />} />
        <Route path="/" element={<List />} />
      </Routes>
    </>
  );
};
