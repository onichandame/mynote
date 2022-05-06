import { FC } from "react";
import { Link } from "react-router-dom";

export const PlaceHolder: FC = () => {
  return (
    <div>
      Your password vault is empty, {<Link to="create">click</Link>} to add your
      first password!
    </div>
  );
};
