import { FC } from "react";
import { Route, Routes } from "react-router-dom";

import { Form } from "../common";
import { Create } from "./create";
import { Detail } from "./detail";
import { List } from "./list";

export const Password: FC = () => {
  return (
    <>
      <Routes>
        <Route path="/" element={<List />} />
        <Route path="/:id/" element={<Detail />} />
        <Route
          path="create"
          element={
            <Form>
              <Create />
            </Form>
          }
        />
      </Routes>
    </>
  );
};
