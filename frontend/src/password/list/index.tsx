import { Add } from "@mui/icons-material";
import {
  CardMedia,
  Card,
  CardActionArea,
  CardContent,
  Grid,
  SpeedDialAction,
  Typography,
} from "@mui/material";
import { FC, useCallback, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";

import { Actions } from "../../actions";
import { useService } from "../../backend";
import { Loading } from "../../common";
import { Password } from "../../model";
import { PlaceHolder } from "./placeholder";

export const List: FC = () => {
  const [pwds, setPwds] = useState<Password[] | null>(null);
  const navigate = useNavigate();
  const svc = useService();
  const updatePwds = useCallback(async () => {
    const pwds = (
      await svc.listPasswords({ deletedAt: { null: true } })
    ).edges.map((v) => v.node);
    setPwds(pwds);
  }, [svc]);
  useEffect(() => {
    updatePwds();
  }, [updatePwds]);
  return pwds ? (
    pwds.length ? (
      <>
        <Grid
          container
          direction="row"
          spacing={3}
          justifyContent="start"
          flexGrow={1}
        >
          {pwds.map((pwd) => (
            <Grid item key={pwd.id}>
              <Card sx={{ minWidth: 275 }}>
                <CardActionArea
                  onClick={() => {
                    navigate(pwd.id.toString());
                  }}
                >
                  <CardMedia
                    component="img"
                    height="150"
                    image="https://media.wired.com/photos/5926e34f8d4ebc5ab806bd1c/master/pass/GettyImages-528338761.jpg"
                    alt={pwd.title}
                  />
                  <CardContent>
                    <Typography variant="h5" align="center">
                      {pwd.title}
                    </Typography>
                  </CardContent>
                </CardActionArea>
              </Card>
            </Grid>
          ))}
        </Grid>
        <Actions>
          <SpeedDialAction
            icon={<Add />}
            tooltipTitle="Create"
            onClick={() => {
              navigate(`create`);
            }}
          />
        </Actions>
      </>
    ) : (
      <PlaceHolder />
    )
  ) : (
    <Loading />
  );
};
