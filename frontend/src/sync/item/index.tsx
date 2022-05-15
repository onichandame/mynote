import { FC, useEffect, useState } from "react";
import { Route, Routes, useParams } from "react-router-dom";

import { useService } from "../../backend";
import { Exception, Loading } from "../../common";
import { Peer } from "../../model";
import { formatError } from "../../util";
import { Detail } from "./detail";
import { Update } from "./update";

export const Item: FC = () => {
  const [peer, setPeer] = useState<Peer | null>(null);
  const [err, setErr] = useState<string | null>(null);
  const svc = useService();
  const params = useParams();
  useEffect(() => {
    if (params.id)
      svc
        .listPeers({
          id: { eq: parseInt(params.id) },
          deletedAt: { null: true },
        })
        .then((conns) => conns.edges.map((v) => v.node))
        .then((peers) => {
          if (peers[0]) setPeer(peers[0]);
        })
        .catch((e) => {
          setErr(formatError(e));
        });
  }, [svc, params]);
  return err ? (
    <Exception message={err} />
  ) : peer ? (
    <Routes>
      <Route path="/" element={<Detail peer={peer} />} />
      <Route path="/update" element={<Update peer={peer} />} />
    </Routes>
  ) : (
    <Loading />
  );
};
