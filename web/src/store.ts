// App state: dice sets + the currently selected set, persisted to localStorage.
// Mutations notify subscribers so the active view can re-render.
import type { DieType, Fav } from "./dice";

export interface StoredDie {
  die: DieType;
  fav: Fav;
}

export interface DiceSet {
  id: string;
  name: string;
  dice: StoredDie[];
}

const SETS_KEY = "dice.sets.v2";
const SELECTED_KEY = "dice.selectedSet.v1";
const LEGACY_KEY = "dice.savedSets.v1"; // pre-redesign shape: { name, dice: DieType[] }[]

let sets: DiceSet[] = [];
let selectedId: string | null = null;
const listeners = new Set<() => void>();

function uid(): string {
  return typeof crypto !== "undefined" && crypto.randomUUID
    ? crypto.randomUUID()
    : `id-${Date.now()}-${Math.random().toString(36).slice(2)}`;
}

function persist() {
  localStorage.setItem(SETS_KEY, JSON.stringify(sets));
  if (selectedId) localStorage.setItem(SELECTED_KEY, selectedId);
  else localStorage.removeItem(SELECTED_KEY);
}

function emit() {
  for (const fn of listeners) fn();
}

function sortByName() {
  sets.sort((a, b) => a.name.localeCompare(b.name));
}

/** Subscribe to state changes (called after every mutation). */
export function subscribe(fn: () => void): void {
  listeners.add(fn);
}

/** Load persisted state, migrating the old flat format if present. */
export function init(): void {
  sets = readSets();
  sortByName();
  selectedId = localStorage.getItem(SELECTED_KEY);
  if (!selectedId || !sets.some((s) => s.id === selectedId)) {
    selectedId = sets[0]?.id ?? null;
  }
}

function readSets(): DiceSet[] {
  const raw = localStorage.getItem(SETS_KEY);
  if (raw) {
    try {
      const parsed = JSON.parse(raw);
      if (Array.isArray(parsed)) return parsed;
    } catch {
      /* fall through to migration */
    }
  }
  return migrateLegacy();
}

function migrateLegacy(): DiceSet[] {
  try {
    const raw = localStorage.getItem(LEGACY_KEY);
    if (!raw) return [];
    const old = JSON.parse(raw) as { name: string; dice: DieType[] }[];
    if (!Array.isArray(old)) return [];
    const migrated: DiceSet[] = old.map((s) => ({
      id: uid(),
      name: s.name,
      dice: s.dice.map((die) => ({ die, fav: "neutral" as Fav })),
    }));
    localStorage.setItem(SETS_KEY, JSON.stringify(migrated));
    return migrated;
  } catch {
    return [];
  }
}

export function getSets(): DiceSet[] {
  return sets;
}

export function getSet(id: string): DiceSet | null {
  return sets.find((s) => s.id === id) ?? null;
}

export function getSelectedId(): string | null {
  return selectedId;
}

export function getSelected(): DiceSet | null {
  return selectedId ? getSet(selectedId) : null;
}

export function selectSet(id: string): void {
  selectedId = id;
  persist();
  emit();
}

export function createSet(name: string, dice: StoredDie[]): DiceSet {
  const set: DiceSet = { id: uid(), name, dice };
  sets.push(set);
  sortByName();
  if (!selectedId) selectedId = set.id;
  persist();
  emit();
  return set;
}

export function updateSet(
  id: string,
  patch: { name?: string; dice?: StoredDie[] },
): void {
  const set = getSet(id);
  if (!set) return;
  if (patch.name !== undefined) set.name = patch.name;
  if (patch.dice !== undefined) set.dice = patch.dice;
  sortByName();
  persist();
  emit();
}

export function deleteSet(id: string): void {
  sets = sets.filter((s) => s.id !== id);
  if (selectedId === id) selectedId = sets[0]?.id ?? null;
  persist();
  emit();
}
