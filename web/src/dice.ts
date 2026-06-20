// Typed wrapper around the WebAssembly dice engine. Everything crosses the
// boundary as JSON strings (see src/wasm_api.rs); this module is the only place
// that deals with that encoding.
import init, {
  roll_dice as wasmRoll,
  parse_expression as wasmParse,
} from "../pkg/dice.js";

export const DIE_TYPES = ["d4", "d6", "d8", "d10", "d12", "d20"] as const;
export type DieType = (typeof DIE_TYPES)[number];
export type Fav = "neutral" | "favoured" | "illfavoured";

export interface DieSpec {
  die: DieType;
  fav?: Fav;
}

export interface RollOutput {
  die: DieType;
  fav: Fav;
  value: number;
}

/** Number of faces for a die type, e.g. "d20" -> 20. */
export function faces(die: DieType): number {
  return Number(die.slice(1));
}

const FAV_CYCLE: Fav[] = ["neutral", "favoured", "illfavoured"];

/** Next favourableness in the cycle neutral -> favoured -> illfavoured -> … */
export function nextFav(fav: Fav): Fav {
  return FAV_CYCLE[(FAV_CYCLE.indexOf(fav) + 1) % FAV_CYCLE.length];
}

export function favSymbol(fav: Fav): string {
  return fav === "favoured" ? "+" : fav === "illfavoured" ? "−" : "=";
}

export function favLabel(fav: Fav): string {
  return fav === "favoured"
    ? "Advantage"
    : fav === "illfavoured"
      ? "Disadvantage"
      : "Neutral";
}

let ready: Promise<void> | null = null;

/** Load + instantiate the wasm module. Safe to call repeatedly; runs once. */
export function initDice(): Promise<void> {
  if (!ready) ready = init().then(() => undefined);
  return ready;
}

/** Roll a set of dice. Engine returns results in the same order as `specs`. */
export function rollDice(specs: DieSpec[]): RollOutput[] {
  return JSON.parse(wasmRoll(JSON.stringify(specs)));
}

/** Parse a textual expression like "3d6 +d20" into dice (values are 0). */
export function parseExpression(input: string): RollOutput[] {
  return JSON.parse(wasmParse(input));
}
