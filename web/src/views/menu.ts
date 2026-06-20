// Hamburger drawer: pick a set to make it current, or jump to set management.
import { getSelectedId, getSets, selectSet } from "../store";
import { navigate } from "../router";
import { escapeHtml, summarise } from "../util";

export function renderDrawer(drawer: HTMLElement, close: () => void): void {
  const sets = getSets();
  const selectedId = getSelectedId();
  drawer.replaceChildren();

  const head = document.createElement("div");
  head.className = "drawer-head";
  head.textContent = "Dice sets";
  drawer.append(head);

  const list = document.createElement("ul");
  list.className = "drawer-list";
  if (sets.length === 0) {
    list.innerHTML = `<li class="drawer-empty">No sets yet.</li>`;
  } else {
    for (const s of sets) {
      const li = document.createElement("li");
      const btn = document.createElement("button");
      btn.className = "drawer-item" + (s.id === selectedId ? " active" : "");
      btn.innerHTML = `<strong>${escapeHtml(s.name)}</strong><span>${summarise(s)}</span>`;
      btn.addEventListener("click", () => {
        selectSet(s.id);
        navigate("/");
        close();
      });
      li.append(btn);
      list.append(li);
    }
  }
  drawer.append(list);

  const manage = document.createElement("button");
  manage.className = "drawer-manage";
  manage.textContent = "⚙  Manage sets";
  manage.addEventListener("click", () => {
    navigate("/sets");
    close();
  });
  drawer.append(manage);
}
