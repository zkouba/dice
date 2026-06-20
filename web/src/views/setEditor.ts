// Editor view: create a new set or edit an existing one. Works on a local copy
// of the dice and only commits to the store on Save.
import { DIE_TYPES, favLabel, favSymbol, nextFav } from "../dice";
import { createSet, getSet, type StoredDie, updateSet } from "../store";
import { navigate } from "../router";

export function renderSetEditor(outlet: HTMLElement, id: string | null): void {
  const existing = id ? getSet(id) : null;
  if (id && !existing) {
    navigate("/sets");
    return;
  }

  let name = existing ? existing.name : "";
  const dice: StoredDie[] = existing ? existing.dice.map((d) => ({ ...d })) : [];

  outlet.replaceChildren();
  const view = document.createElement("div");
  view.className = "editor";
  view.innerHTML = `
    <h2>${existing ? "Edit set" : "New set"}</h2>
    <label class="field">
      <span>Name</span>
      <input id="name" type="text" placeholder="Set name…" autocomplete="off" />
    </label>
    <h3 class="section-title">Add dice</h3>
    <div class="palette" id="palette"></div>
    <h3 class="section-title">Dice in this set</h3>
    <div class="editor-dice" id="dice"></div>
    <div class="actions">
      <button class="btn btn-primary" id="save">Save</button>
      <button class="btn" id="cancel">Cancel</button>
    </div>`;
  outlet.append(view);

  const nameInput = view.querySelector<HTMLInputElement>("#name")!;
  nameInput.value = name;
  nameInput.addEventListener("input", () => {
    name = nameInput.value;
    nameInput.classList.remove("invalid");
  });

  const palette = view.querySelector("#palette")!;
  for (const t of DIE_TYPES) {
    const b = document.createElement("button");
    b.className = `die-btn die-${t}`;
    b.textContent = t;
    b.title = `Add a ${t}`;
    b.addEventListener("click", () => {
      dice.push({ die: t, fav: "neutral" });
      renderDice();
    });
    palette.append(b);
  }

  const diceBox = view.querySelector("#dice")!;
  function renderDice() {
    diceBox.replaceChildren();
    if (dice.length === 0) {
      diceBox.innerHTML = `<p class="tray-hint">No dice yet — click a die above.</p>`;
      return;
    }
    dice.forEach((d, index) => {
      const row = document.createElement("div");
      row.className = `die-row die-${d.die}`;

      const chip = document.createElement("span");
      chip.className = "die-chip";
      chip.textContent = d.die;

      const favBtn = document.createElement("button");
      favBtn.className = `fav-toggle fav-${d.fav}`;
      favBtn.textContent = `${favSymbol(d.fav)} ${favLabel(d.fav)}`;
      favBtn.title = "Cycle favourableness";
      favBtn.addEventListener("click", () => {
        dice[index] = { ...d, fav: nextFav(d.fav) };
        renderDice();
      });

      const remove = document.createElement("button");
      remove.className = "die-remove-row";
      remove.textContent = "Remove";
      remove.addEventListener("click", () => {
        dice.splice(index, 1);
        renderDice();
      });

      row.append(chip, favBtn, remove);
      diceBox.append(row);
    });
  }
  renderDice();

  view
    .querySelector("#cancel")!
    .addEventListener("click", () => navigate("/sets"));
  view.querySelector("#save")!.addEventListener("click", () => {
    const trimmed = name.trim();
    if (!trimmed) {
      nameInput.classList.add("invalid");
      nameInput.focus();
      return;
    }
    if (existing) updateSet(existing.id, { name: trimmed, dice });
    else createSet(trimmed, dice);
    navigate("/sets");
  });
}
