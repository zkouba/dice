// Roll animation. The engine has already decided the real value; this just plays
// a decelerating "shuffle" of random faces that lands on `finalValue`, so the
// visual always agrees with the authoritative result.

/**
 * Animate one die element to its final value.
 * @param el         the die's number element
 * @param faceCount  how many faces (for the random shuffle range)
 * @param finalValue the value the engine rolled
 * @returns a promise that resolves when the die has settled
 */
export function animateDie(
  el: HTMLElement,
  faceCount: number,
  finalValue: number,
): Promise<void> {
  return new Promise((resolve) => {
    el.classList.remove("settled");
    el.classList.add("rolling");

    let delay = 45; // grows each step, so the shuffle slows to a stop
    const step = () => {
      if (delay < 280) {
        el.textContent = String(1 + Math.floor(Math.random() * faceCount));
        delay *= 1.18;
        window.setTimeout(step, delay);
      } else {
        el.textContent = String(finalValue);
        el.classList.remove("rolling");
        el.classList.add("settled");
        window.setTimeout(() => el.classList.remove("settled"), 350);
        resolve();
      }
    };
    window.setTimeout(step, delay);
  });
}
