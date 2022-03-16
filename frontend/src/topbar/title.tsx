import { FC } from "react";
import { Link } from "react-router-dom";

export const Title: FC = () => {
  return (
    <Link to="/" style={{ textDecoration: `none`, color: `inherit` }}>
      My Note
    </Link>
  );
};
