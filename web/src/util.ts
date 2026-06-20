import { DIE_TYPES, type DieType } from "./dice";
import type { DiceSet } from "./store";

/** Escape user text for safe insertion via innerHTML. */
export function escapeHtml(s: string): string {
  const div = document.createElement("div");
  div.textContent = s;
  return div.innerHTML;
}

/** Compact description of a set's dice, e.g. "1d6 + 1d8 + 1d20". */
export function summarise(set: DiceSet): string {
  if (set.dice.length === 0) return "empty";
  const counts = new Map<DieType, number>();
  for (const d of set.dice) counts.set(d.die, (counts.get(d.die) ?? 0) + 1);
  return DIE_TYPES.filter((t) => counts.has(t))
    .map((t) => `${counts.get(t)}${t}`)
    .join(" + ");
}
