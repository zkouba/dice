//! WebAssembly boundary for the dice engine.
//!
//! This module is the *only* place that knows about the browser. It exposes a
//! small, stable JSON contract (`DieSpec` in, `RollOutput` out) and delegates
//! every roll to the untouched `dice` engine, so the rolling logic stays the
//! single source of truth shared with the native binary.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::dice::error::DiceError;
use crate::dice::parser::parse_dice_roll;
use crate::dice::rolls::{roll_fav, DiceRoll, DiceType, Favourableness};

/// A single die the frontend wants rolled, e.g. `{ "die": "d6", "fav": "favoured" }`.
#[derive(Deserialize)]
struct DieSpec {
    /// Die size as `"d4"`, `"d6"`, … matching [`DiceType`]'s `Display`.
    die: String,
    /// `"neutral"` (default), `"favoured"` or `"illfavoured"`. Optional.
    fav: Option<String>,
}

/// The result of rolling one die, returned to the frontend.
#[derive(Serialize)]
struct RollOutput {
    die: String,
    fav: String,
    value: u8,
}

/// Roll a set of dice. Takes a JSON array of `DieSpec`, returns a JSON array of
/// `RollOutput` in the same order. The frontend animates *towards* these values.
#[wasm_bindgen]
pub fn roll_dice(specs_json: &str) -> Result<String, JsValue> {
    let specs: Vec<DieSpec> = serde_json::from_str(specs_json).map_err(to_js_err)?;

    let mut out = Vec::with_capacity(specs.len());
    for spec in specs {
        let die_type = parse_die_type(&spec.die).map_err(to_js_err)?;
        let fav = parse_fav(spec.fav.as_deref()).map_err(to_js_err)?;
        let result = roll_fav(DiceRoll {
            die_type,
            favourableness: fav,
        });
        out.push(RollOutput {
            die: spec.die,
            fav: fav_to_str(fav).to_string(),
            value: result.value,
        });
    }

    serde_json::to_string(&out).map_err(to_js_err)
}

/// Parse a textual expression (e.g. `"3d6 +d20"`) into a JSON array of `DieSpec`,
/// reusing the engine's parser. Lets the frontend offer free-text entry on top of
/// the click-to-add tray.
#[wasm_bindgen]
pub fn parse_expression(input: &str) -> Result<String, JsValue> {
    let rolls = parse_dice_roll(input).map_err(to_js_err)?;
    let specs: Vec<RollOutput> = rolls
        .into_iter()
        .map(|r| RollOutput {
            die: r.die_type.to_string(),
            fav: fav_to_str(r.favourableness).to_string(),
            value: 0,
        })
        .collect();
    serde_json::to_string(&specs).map_err(to_js_err)
}

fn parse_die_type(die: &str) -> Result<DiceType, DiceError> {
    Ok(match die {
        "d2" => DiceType::D2,
        "d4" => DiceType::D4,
        "d6" => DiceType::D6,
        "d8" => DiceType::D8,
        "d10" => DiceType::D10,
        "d12" => DiceType::D12,
        "d20" => DiceType::D20,
        "d100" => DiceType::D100,
        other => return Err(DiceError::new_standalone(format!("Invalid die type: {other}"))),
    })
}

fn parse_fav(fav: Option<&str>) -> Result<Favourableness, DiceError> {
    Ok(match fav {
        None | Some("neutral") | Some("") => Favourableness::Neutral(false),
        Some("favoured") => Favourableness::Favoured,
        Some("illfavoured") => Favourableness::Illfavoured,
        Some(other) => {
            return Err(DiceError::new_standalone(format!(
                "Invalid favourableness: {other}"
            )))
        }
    })
}

fn fav_to_str(fav: Favourableness) -> &'static str {
    match fav {
        Favourableness::Favoured => "favoured",
        Favourableness::Illfavoured => "illfavoured",
        Favourableness::Neutral(_) => "neutral",
    }
}

fn to_js_err<E: std::fmt::Display>(e: E) -> JsValue {
    JsValue::from_str(&e.to_string())
}
