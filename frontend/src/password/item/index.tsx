import { FC, useCallback, useEffect, useState } from "react";
import { Route, Routes, useParams } from "react-router-dom";

import { useService } from "../../backend";
import { Loading } from "../../common";
import { Password } from "../../model";
import { Detail } from "./detail";
import { Update } from "./update";

export const Item: FC = () => {
  const svc = useService();
  const params = useParams();
  const [pwd, setPwd] = useState<Password | null>(null);
  const updatePwd = useCallback(async () => {
    if (params.id) {
      const pwd = (
        await svc.listPasswords({
          id: { eq: parseInt(params.id) },
          deletedAt: { null: true },
        })
      ).edges[0].node;
      setPwd(pwd);
    }
  }, [svc, params]);
  useEffect(() => {
    updatePwd();
  }, [updatePwd]);
  return pwd ? (
    <Routes>
      <Route path="/update" element={<Update pwd={pwd} />} />
      <Route path="/" element={<Detail pwd={pwd} />} />
    </Routes>
  ) : (
    <Loading />
  );
};
