import { FC } from "react";
import { Route, Routes } from "react-router-dom";

import { Create } from "./create";
import { Detail } from "./item";
import { List } from "./list";

export const Sync: FC = () => {
  return (
    <Routes>
      <Route path="/" element={<List />} />
      <Route path="/create" element={<Create />} />
      <Route path="/:id" element={<Detail />} />
    </Routes>
  );
};
