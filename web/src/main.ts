import "./styles.css";
import {
  DIE_TYPES,
  type DieType,
  faces,
  initDice,
  rollDice,
} from "./dice";
import { animateDie } from "./animation";
import { type DiceSet, deleteSet, loadSets, saveSet } from "./sets";

// --- State -----------------------------------------------------------------

let tray: DieType[] = []; // dice queued for the next roll, in order
let sets: DiceSet[] = loadSets();
let rolling = false;

// --- Element handles -------------------------------------------------------

const $ = <T extends HTMLElement>(id: string): T => {
  const el = document.getElementById(id);
  if (!el) throw new Error(`Missing #${id}`);
  return el as T;
};

const paletteEl = $("palette");
const trayEl = $("tray");
const totalEl = $("total");
const setListEl = $("setList");
const rollBtn = $<HTMLButtonElement>("roll");
const clearBtn = $<HTMLButtonElement>("clear");
const saveForm = $<HTMLFormElement>("saveForm");
const setNameInput = $<HTMLInputElement>("setName");

// --- Rendering -------------------------------------------------------------

function renderPalette() {
  paletteEl.innerHTML = "";
  for (const die of DIE_TYPES) {
    const btn = document.createElement("button");
    btn.className = `die-btn die-${die}`;
    btn.textContent = die;
    btn.title = `Add a ${die}`;
    btn.addEventListener("click", () => addDie(die));
    paletteEl.append(btn);
  }
}

function renderTray() {
  trayEl.innerHTML = "";
  if (tray.length === 0) {
    const hint = document.createElement("p");
    hint.className = "tray-hint";
    hint.textContent = "Click a die above to add it to your roll.";
    trayEl.append(hint);
  } else {
    tray.forEach((die, index) => {
      const chip = document.createElement("div");
      chip.className = `die die-${die}`;

      const value = document.createElement("span");
      value.className = "die-value";
      value.dataset.index = String(index);
      value.textContent = die;

      const remove = document.createElement("button");
      remove.className = "die-remove";
      remove.textContent = "×";
      remove.title = "Remove";
      remove.addEventListener("click", () => removeDie(index));

      chip.append(value, remove);
      trayEl.append(chip);
    });
  }
  syncControls();
}

function renderSets() {
  setListEl.innerHTML = "";
  if (sets.length === 0) {
    const empty = document.createElement("li");
    empty.className = "set-empty";
    empty.textContent = "No saved sets yet.";
    setListEl.append(empty);
    return;
  }
  for (const set of sets) {
    const li = document.createElement("li");
    li.className = "set-item";

    const load = document.createElement("button");
    load.className = "set-load";
    load.innerHTML = `<strong>${escapeHtml(set.name)}</strong><span>${summarise(set.dice)}</span>`;
    load.title = "Load this set";
    load.addEventListener("click", () => loadSet(set));

    const del = document.createElement("button");
    del.className = "set-delete";
    del.textContent = "Delete";
    del.addEventListener("click", () => {
      sets = deleteSet(sets, set.name);
      renderSets();
    });

    li.append(load, del);
    setListEl.append(li);
  }
}

function syncControls() {
  const hasDice = tray.length > 0;
  rollBtn.disabled = !hasDice || rolling;
  clearBtn.disabled = !hasDice || rolling;
}

// --- Actions ---------------------------------------------------------------

function addDie(die: DieType) {
  tray.push(die);
  totalEl.textContent = "";
  renderTray();
}

function removeDie(index: number) {
  tray.splice(index, 1);
  totalEl.textContent = "";
  renderTray();
}

function clearTray() {
  tray = [];
  totalEl.textContent = "";
  renderTray();
}

async function roll() {
  if (rolling || tray.length === 0) return;
  rolling = true;
  totalEl.textContent = "";
  syncControls();

  // Ask the Rust engine for the authoritative results, then animate towards them.
  const results = rollDice(tray.map((die) => ({ die })));
  const valueEls = trayEl.querySelectorAll<HTMLElement>(".die-value");

  await Promise.all(
    results.map((result, i) => {
      const el = valueEls[i];
      return el ? animateDie(el, faces(result.die), result.value) : Promise.resolve();
    }),
  );

  const total = results.reduce((sum, r) => sum + r.value, 0);
  totalEl.textContent = `Total: ${total}`;
  rolling = false;
  syncControls();
}

function loadSet(set: DiceSet) {
  tray = [...set.dice];
  totalEl.textContent = "";
  renderTray();
}

function onSave(event: SubmitEvent) {
  event.preventDefault();
  const name = setNameInput.value.trim();
  if (!name || tray.length === 0) return;
  sets = saveSet(sets, name, tray);
  setNameInput.value = "";
  renderSets();
}

// --- Helpers ---------------------------------------------------------------

function summarise(dice: DieType[]): string {
  const counts = new Map<DieType, number>();
  for (const d of dice) counts.set(d, (counts.get(d) ?? 0) + 1);
  return DIE_TYPES.filter((d) => counts.has(d))
    .map((d) => `${counts.get(d)}${d}`)
    .join(" + ");
}

function escapeHtml(s: string): string {
  const div = document.createElement("div");
  div.textContent = s;
  return div.innerHTML;
}

// --- Bootstrap -------------------------------------------------------------

async function main() {
  renderPalette();
  renderTray();
  renderSets();

  rollBtn.addEventListener("click", roll);
  clearBtn.addEventListener("click", clearTray);
  saveForm.addEventListener("submit", onSave);

  await initDice(); // load the wasm engine before the first roll
}

main();
