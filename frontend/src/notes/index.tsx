import { FC } from "react";
import { Routes, Route } from "react-router-dom";

import { Create } from "./create";
import { Note } from "./note";
import { List } from "./list";
import { Form } from "../common";
import { Sync } from "./sync";

export const Notes: FC = () => {
  return (
    <>
      <Routes>
        <Route
          path="create"
          element={
            <Form>
              <Create />
            </Form>
          }
        />
        <Route
          path="sync"
          element={
            <Form>
              <Sync />
            </Form>
          }
        />
        <Route path=":id/*" element={<Note />} />
        <Route path="/" element={<List />} />
      </Routes>
    </>
  );
};
