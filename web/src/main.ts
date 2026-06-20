import "./styles.css";
import { initDice } from "./dice";
import * as store from "./store";
import { onRouteChange, parseRoute } from "./router";
import { renderRoller } from "./views/roller";
import { renderSetList } from "./views/setList";
import { renderSetEditor } from "./views/setEditor";
import { renderDrawer } from "./views/menu";

const $ = (id: string): HTMLElement => {
  const el = document.getElementById(id);
  if (!el) throw new Error(`Missing #${id}`);
  return el;
};

const outlet = $("outlet");
const drawer = $("drawer");
const scrim = $("scrim");
const menuBtn = $("menuBtn");
const topTitle = $("topTitle");

function openDrawer() {
  renderDrawer(drawer, closeDrawer);
  drawer.classList.add("open");
  scrim.hidden = false;
}

function closeDrawer() {
  drawer.classList.remove("open");
  scrim.hidden = true;
}

function render() {
  const route = parseRoute();
  switch (route.name) {
    case "roller":
      topTitle.textContent = "Dice Roller";
      renderRoller(outlet);
      break;
    case "list":
      topTitle.textContent = "Manage sets";
      renderSetList(outlet);
      break;
    case "editor":
      topTitle.textContent = route.id ? "Edit set" : "New set";
      renderSetEditor(outlet, route.id);
      break;
  }
}

menuBtn.addEventListener("click", openDrawer);
scrim.addEventListener("click", closeDrawer);
onRouteChange(() => {
  closeDrawer();
  render();
});
// Re-render the active view whenever stored state changes.
store.subscribe(render);

store.init();
render();
void initDice(); // load the wasm engine in the background
