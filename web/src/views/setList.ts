// Management view: list every set with select / edit / delete, plus "new set".
import { deleteSet, getSets, selectSet } from "../store";
import { navigate } from "../router";
import { escapeHtml, summarise } from "../util";

export function renderSetList(outlet: HTMLElement): void {
  const sets = getSets();
  outlet.replaceChildren();

  const view = document.createElement("div");
  view.className = "manage";
  view.innerHTML = `
    <div class="manage-head">
      <h2>Manage sets</h2>
      <button class="btn btn-primary" id="newSet">New set</button>
    </div>`;

  const list = document.createElement("ul");
  list.className = "set-list";
  if (sets.length === 0) {
    list.innerHTML = `<li class="set-empty">No sets yet — create one.</li>`;
  } else {
    for (const s of sets) {
      const li = document.createElement("li");
      li.className = "set-item";
      li.innerHTML = `
        <div class="set-info">
          <strong>${escapeHtml(s.name)}</strong>
          <span>${summarise(s)}</span>
        </div>
        <div class="set-actions"></div>`;

      const actions = li.querySelector(".set-actions")!;
      actions.append(
        button("Select & roll", "btn-sm", () => {
          selectSet(s.id);
          navigate("/");
        }),
        button("Edit", "btn-sm", () => navigate(`/sets/${s.id}`)),
        button("Delete", "btn-sm btn-danger", () => {
          if (confirm(`Delete "${s.name}"?`)) deleteSet(s.id);
        }),
      );
      list.append(li);
    }
  }

  view.append(list);
  outlet.append(view);
  view
    .querySelector("#newSet")!
    .addEventListener("click", () => navigate("/sets/new"));
}

function button(label: string, cls: string, onClick: () => void): HTMLButtonElement {
  const b = document.createElement("button");
  b.className = `btn ${cls}`;
  b.textContent = label;
  b.addEventListener("click", onClick);
  return b;
}
