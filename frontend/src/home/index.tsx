import { Sync } from "@mui/icons-material";
import {
  Card,
  CardActionArea,
  CardContent,
  CardMedia,
  Grid,
  SpeedDialAction,
  Typography,
} from "@mui/material";
import { FC } from "react";
import { useNavigate } from "react-router-dom";

import noteImg from "./note.png";
import { Actions } from "../actions";

const cards = [
  {
    image: noteImg,
    title: `Notes`,
    url: `/notes`,
  },
] as { url: string; image: string; title: string }[];

export const Home: FC = () => {
  const navigate = useNavigate();
  return (
    <>
      <Grid container direction="row" spacing={3}>
        {cards.map((card) => (
          <Grid item key={card.url}>
            <Card
              sx={{
                maxWidth: 345,
              }}
              variant="outlined"
            >
              <CardActionArea
                onClick={() => {
                  navigate(card.url);
                }}
              >
                <CardMedia
                  component="img"
                  height="140"
                  image={card.image}
                  alt={card.title}
                />
                <CardContent>
                  <Typography gutterBottom variant="h5" component="div">
                    {card.title}
                  </Typography>
                </CardContent>
              </CardActionArea>
            </Card>
          </Grid>
        ))}
      </Grid>
      <Actions>
        <SpeedDialAction
          icon={<Sync />}
          tooltipTitle="Sync from"
          onClick={() => {
            navigate(`sync`);
          }}
        />
      </Actions>
    </>
  );
};
