// Saved dice sets, persisted to localStorage as JSON.
import type { DieType } from "./dice";

export interface DiceSet {
  name: string;
  dice: DieType[];
}

const STORAGE_KEY = "dice.savedSets.v1";

export function loadSets(): DiceSet[] {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return [];
    const parsed = JSON.parse(raw);
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

function persist(sets: DiceSet[]): void {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(sets));
}

/** Insert or overwrite a set by name (case-insensitive). Returns the new list. */
export function saveSet(sets: DiceSet[], name: string, dice: DieType[]): DiceSet[] {
  const trimmed = name.trim();
  const next = sets.filter((s) => s.name.toLowerCase() !== trimmed.toLowerCase());
  next.push({ name: trimmed, dice: [...dice] });
  next.sort((a, b) => a.name.localeCompare(b.name));
  persist(next);
  return next;
}

export function deleteSet(sets: DiceSet[], name: string): DiceSet[] {
  const next = sets.filter((s) => s.name !== name);
  persist(next);
  return next;
}
