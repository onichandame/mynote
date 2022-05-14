import { FC, useCallback, useEffect, useState } from "react";
import { Route, Routes, useParams } from "react-router-dom";

import { useService } from "../../backend";
import { Exception, Loading } from "../../common";
import { Password } from "../../model";
import { formatError } from "../../util";
import { Detail } from "./detail";
import { Update } from "./update";

export const Item: FC = () => {
  const svc = useService();
  const params = useParams();
  const [pwd, setPwd] = useState<Password | null>(null);
  const [err, setErr] = useState<string | null>(null);
  const updatePwd = useCallback(async () => {
    if (params.id) {
      try {
        const pwd = (
          await svc.listPasswords({
            id: { eq: parseInt(params.id) },
            deletedAt: { null: true },
          })
        ).edges[0]?.node;
        if (!pwd) throw new Error(`password not found`);
        setPwd(pwd);
      } catch (e) {
        setErr(formatError(e));
      }
    }
  }, [svc, params]);
  useEffect(() => {
    updatePwd();
  }, [updatePwd]);
  return err ? (
    <Exception code={404} message={err} />
  ) : pwd ? (
    <Routes>
      <Route path="/update" element={<Update pwd={pwd} />} />
      <Route path="/" element={<Detail pwd={pwd} />} />
    </Routes>
  ) : (
    <Loading />
  );
};
