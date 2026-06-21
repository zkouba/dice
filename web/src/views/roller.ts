// Home view: shows the currently selected set and lets the user roll it in a
// big 3D tray where the dice tumble around and bounce off each other.
import { rollDice } from "../dice";
import { DiceTray } from "../dice3d";
import { getSelected } from "../store";
import { navigate } from "../router";
import { escapeHtml, summarise } from "../util";

let rolling = false;
// The tray currently mounted. Disposed and rebuilt on every re-render so we
// never leak GPU resources across navigations / store updates.
let tray: DiceTray | null = null;

export function renderRoller(outlet: HTMLElement): void {
  // Tear down any tray from a previous render before replacing the DOM.
  tray?.dispose();
  tray = null;

  const set = getSelected();
  outlet.replaceChildren();

  const view = document.createElement("div");
  view.className = "roller";

  if (!set) {
    view.innerHTML = `
      <div class="empty-state">
        <p>No dice sets yet.</p>
        <button class="btn btn-primary" id="goCreate">Create your first set</button>
      </div>`;
    outlet.append(view);
    view
      .querySelector("#goCreate")!
      .addEventListener("click", () => navigate("/sets/new"));
    return;
  }

  const head = document.createElement("div");
  head.className = "roller-head";
  head.innerHTML = `<h2>${escapeHtml(set.name)}</h2><p class="muted">${summarise(set)}</p>`;

  const trayEl = document.createElement("div");
  trayEl.className = "tray";

  const actions = document.createElement("div");
  actions.className = "actions";
  const rollBtn = document.createElement("button");
  rollBtn.className = "btn btn-primary btn-roll";
  rollBtn.textContent = "Roll";
  rollBtn.disabled = set.dice.length === 0;
  const total = document.createElement("span");
  total.className = "total";
  actions.append(rollBtn, total);

  view.append(head, trayEl, actions);
  outlet.append(view);

  if (set.dice.length === 0) {
    trayEl.innerHTML = `<p class="tray-hint">This set has no dice. Edit it to add some.</p>`;
    return;
  }

  // The tray canvas fills the box; DiceTray watches it for resizes.
  const canvas = document.createElement("canvas");
  canvas.className = "tray-canvas";
  trayEl.append(canvas);
  const instance = new DiceTray(
    canvas,
    set.dice.map((d) => ({ die: d.die, fav: d.fav })),
  );
  tray = instance;

  rollBtn.addEventListener("click", async () => {
    if (rolling) return;
    rolling = true;
    rollBtn.disabled = true;
    total.textContent = "";

    const results = rollDice(set.dice.map((d) => ({ die: d.die, fav: d.fav })));
    await instance.roll(results.map((r) => r.value));

    total.textContent = `Total: ${results.reduce((s, r) => s + r.value, 0)}`;
    rolling = false;
    rollBtn.disabled = false;
  });
}
