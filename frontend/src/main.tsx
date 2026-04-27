import { render } from "solid-js/web";

import { DdgcApp } from "./app/DdgcApp";
import "./styles.css";

const root = document.getElementById("root");

if (!root) {
  throw new Error("frontend root container not found");
}

render(() => <DdgcApp />, root);