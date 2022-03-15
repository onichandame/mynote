import {
  IconButton,
  CardHeader,
  Card,
  Menu,
  MenuItem,
  CardActionArea,
  CardContent,
  Typography,
} from "@mui/material";
import { MoreVert } from "@mui/icons-material";
import { FC, useState } from "react";
import { useNavigate } from "react-router-dom";

import { Note } from "../../model";

export const Item: FC<{ item: Note }> = ({ item }) => {
  const navigate = useNavigate();
  const [menuAnchor, setMenuAnchor] = useState<HTMLElement | null>(null);
  return (
    <>
      <Card sx={{ minWidth: 275 }} variant="outlined">
        <CardHeader
          action={
            <IconButton
              onClick={(e) => {
                setMenuAnchor(e.currentTarget);
              }}
            >
              <MoreVert />
            </IconButton>
          }
          title={item.title}
          subheader={new Date(item.createdAt).toLocaleDateString}
        />
        <CardActionArea
          onClick={() => {
            navigate(item.id.toString());
          }}
        >
          <CardContent>
            <Typography>
              {item.content.length < 10
                ? item.content
                : `${item.content.slice(0, 7)}...`}
            </Typography>
          </CardContent>
        </CardActionArea>
      </Card>
      <Menu
        anchorEl={menuAnchor}
        keepMounted
        open={!!menuAnchor}
        anchorOrigin={{ horizontal: `center`, vertical: `bottom` }}
        transformOrigin={{ vertical: `top`, horizontal: `center` }}
        onClose={() => setMenuAnchor(null)}
      >
        <MenuItem
          onClick={() => {
            setMenuAnchor(null);
            navigate(`${item.id}/delete`);
          }}
          sx={{ color: (theme) => theme.palette.secondary.main }}
        >
          Delete
        </MenuItem>
      </Menu>
    </>
  );
};
