// Home view: shows the currently selected set and lets the user roll it.
import { faces, favLabel, favSymbol, rollDice } from "../dice";
import { animateDie } from "../animation";
import { getSelected } from "../store";
import { navigate } from "../router";
import { escapeHtml, summarise } from "../util";

let rolling = false;

export function renderRoller(outlet: HTMLElement): void {
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
  if (set.dice.length === 0) {
    tray.innerHTML = `<p class="tray-hint">This set has no dice. Edit it to add some.</p>`;
  } else {
    for (const d of set.dice) {
      const chip = document.createElement("div");
      chip.className = `die die-${d.die}`;

      const value = document.createElement("span");
      value.className = "die-value";
      value.textContent = d.die;
      chip.append(value);

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
    const valueEls = tray.querySelectorAll<HTMLElement>(".die-value");
    await Promise.all(
      results.map((r, i) =>
        valueEls[i]
          ? animateDie(valueEls[i], faces(r.die), r.value)
          : Promise.resolve(),
      ),
    );

    total.textContent = `Total: ${results.reduce((s, r) => s + r.value, 0)}`;
    rolling = false;
    rollBtn.disabled = false;
  });
}
