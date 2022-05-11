import { Grid, Typography } from "@mui/material";
import { FC, useEffect, useState } from "react";
import { Link } from "react-router-dom";

import { useService } from "../../backend";
import { Loading } from "../../common";
import { Peer } from "../../model";
import { Item } from "./item";

export const List: FC = () => {
  const [peers, setPeers] = useState<Peer[] | null>(null);
  const svc = useService();
  useEffect(() => {
    svc
      .listPeers({ deletedAt: { null: true } })
      .then((conns) => conns.edges.map((v) => v.node))
      .then((peers) => setPeers(peers));
  }, [svc]);
  return peers ? (
    peers.length ? (
      <Grid container direction="column" spacing={1}>
        <Grid item>
          <Typography variant="h5" textAlign="center">
            Select a peer to sync from
          </Typography>
        </Grid>
        <Grid item>
          <Grid container direction="row" spacing={3} justifyContent="start">
            {peers.map((peer) => (
              <Grid item key={peer.id}>
                <Item peer={peer} />
              </Grid>
            ))}
          </Grid>
        </Grid>
      </Grid>
    ) : (
      <Typography variant="h5">
        You don't have peers yet. <Link to="./create">Add</Link> a peer to start
        syncing!
      </Typography>
    )
  ) : (
    <Loading />
  );
};
