import React, { FC, ReactNode } from "react";
import { useStyles } from "./COMPONENT.styles";

interface Props {
  children: ReactNode;
}

const COMPONENT: FC<Props> = ({ children }) => {
  const classes = useStyles();
  return <div className={classes.wrapper}>{children}</div>;
};

export { COMPONENT };
