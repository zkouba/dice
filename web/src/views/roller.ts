// Home view: shows the currently selected set and lets the user roll it.
import { favLabel, favSymbol, rollDice } from "../dice";
import { Die3D } from "../dice3d";
import { getSelected } from "../store";
import { navigate } from "../router";
import { escapeHtml, summarise } from "../util";

const DIE_PX = 96; // on-screen size of each 3D die

let rolling = false;
// The 3D dice currently mounted in the tray. Disposed and rebuilt on every
// re-render so we never leak GPU resources across navigations / store updates.
let mounted: Die3D[] = [];

export function renderRoller(outlet: HTMLElement): void {
  // Tear down any dice from a previous render before replacing the DOM.
  for (const d of mounted) d.dispose();
  mounted = [];

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

  const tray = document.createElement("div");
  tray.className = "tray";
  const dice: Die3D[] = [];
  if (set.dice.length === 0) {
    tray.innerHTML = `<p class="tray-hint">This set has no dice. Edit it to add some.</p>`;
  } else {
    for (const d of set.dice) {
      const chip = document.createElement("div");
      chip.className = `die die-${d.die}`;

      const canvas = document.createElement("canvas");
      canvas.className = "die-canvas";
      chip.append(canvas);
      dice.push(new Die3D(d.die, canvas, DIE_PX));

      if (d.fav !== "neutral") {
        const badge = document.createElement("span");
        badge.className = `fav-badge fav-${d.fav}`;
        badge.textContent = favSymbol(d.fav);
        badge.title = favLabel(d.fav);
        chip.append(badge);
      }
      tray.append(chip);
    }
  }
  mounted = dice;

  const actions = document.createElement("div");
  actions.className = "actions";
  const rollBtn = document.createElement("button");
  rollBtn.className = "btn btn-primary btn-roll";
  rollBtn.textContent = "Roll";
  rollBtn.disabled = set.dice.length === 0;
  const total = document.createElement("span");
  total.className = "total";
  actions.append(rollBtn, total);

  view.append(head, tray, actions);
  outlet.append(view);

  rollBtn.addEventListener("click", async () => {
    if (rolling || set.dice.length === 0) return;
    rolling = true;
    rollBtn.disabled = true;
    total.textContent = "";

    const results = rollDice(set.dice.map((d) => ({ die: d.die, fav: d.fav })));
    await Promise.all(
      results.map((r, i) =>
        dice[i] ? dice[i].roll(r.value) : Promise.resolve(),
      ),
    );

    total.textContent = `Total: ${results.reduce((s, r) => s + r.value, 0)}`;
    rolling = false;
    rollBtn.disabled = false;
  });
}
