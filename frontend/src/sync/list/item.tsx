import {
  Card,
  CardActionArea,
  CardContent,
  CardMedia,
  Typography,
} from "@mui/material";
import { FC } from "react";
import { useNavigate } from "react-router-dom";

import { Peer } from "../../model";

export const Item: FC<{ peer: Peer }> = ({ peer }) => {
  const navigate = useNavigate();
  return (
    <Card sx={{ minWidth: 180 }}>
      <CardActionArea
        onClick={() => {
          navigate(peer.id.toString());
        }}
      >
        <CardMedia
          component="img"
          height="150"
          width="50%"
          image={
            peer.icon ||
            "https://www.vmware.com/content/dam/digitalmarketing/vmware/en/images/gallery/icons/icon-hp-multi-cloud.svg"
          }
          alt={peer.title}
        />
        <CardContent>
          <Typography variant="h5" align="center">
            {peer.title}
          </Typography>
        </CardContent>
      </CardActionArea>
    </Card>
  );
};
