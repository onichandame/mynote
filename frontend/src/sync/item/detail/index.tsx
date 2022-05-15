import { Cloud, CloudOff } from "@mui/icons-material";
import { Avatar, Button, Grid, Typography } from "@mui/material";
import { useSnackbar } from "notistack";
import { FC } from "react";
import { useNavigate } from "react-router-dom";

import { useService } from "../../../backend";
import { CenterRow } from "../../../common";
import { Peer } from "../../../model";
import { Delete } from "./delete";

export const Detail: FC<{ peer: Peer }> = ({ peer }) => {
  const { enqueueSnackbar } = useSnackbar();
  const svc = useService();
  const navigate = useNavigate();
  return (
    <CenterRow>
      <Grid container direction="column" alignItems="stretch" spacing={2}>
        <Grid item>
          <Grid
            container
            direction="row"
            spacing={2}
            alignItems="center"
            justifyContent="center"
          >
            <Grid item>
              <Avatar src={peer.icon} alt={peer.title} />
            </Grid>
            <Grid item>
              <Typography variant="h3" textAlign="center">
                {peer.title}
              </Typography>
            </Grid>
          </Grid>
        </Grid>
        <Grid item>
          <Grid container direction="row" spacing={2} justifyContent="center">
            <Grid item>
              {peer.autoSync ? (
                <Cloud color="info" />
              ) : (
                <CloudOff color="disabled" />
              )}
            </Grid>
            <Grid item>{peer.autoSync ? `Auto Sync on` : `Auto Sync off`}</Grid>
          </Grid>
        </Grid>
        <Grid item>
          <Grid
            container
            direction="row"
            justifyContent="space-between"
            spacing={2}
          >
            <Grid item>
              <Button
                variant="contained"
                color="primary"
                onClick={() => {
                  navigate(`update`);
                }}
              >
                edit
              </Button>
            </Grid>
            <Grid item>
              <Button
                variant="contained"
                color="success"
                onClick={async () => {
                  await svc.syncFromPeer(peer.id);
                  enqueueSnackbar(`sync successful`, {
                    variant: `success`,
                  });
                }}
              >
                sync
              </Button>
            </Grid>
            <Grid item>
              <Delete peer={peer} />
            </Grid>
          </Grid>
        </Grid>
      </Grid>
    </CenterRow>
  );
};
