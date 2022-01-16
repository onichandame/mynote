import {
  Card,
  CardActionArea,
  CardContent,
  CardMedia,
  Grid,
  Typography,
} from "@mui/material";
import { FC } from "react";
import { useNavigate } from "react-router-dom";

const cards = [
  {
    image: `https://vivaldi.com/wp-content/uploads/note-taking-apps-980x551.png`,
    title: `Notes`,
    url: `/notes`,
  },
] as { url: string; image: string; title: string }[];

export const Home: FC = () => {
  const navigate = useNavigate();
  return (
    <Grid container direction="row" spacing={3}>
      {cards.map((card) => (
        <Grid item key={card.url}>
          <Card
            sx={{
              maxWidth: 345,
            }}
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
  );
};
