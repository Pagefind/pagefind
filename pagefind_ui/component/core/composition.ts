const COMPOSITION_TAIL_MS = 50;

/**
 * Track IME composition on an input, and report whether a given keydown
 * belongs to that composition.
 *
 * Typing Japanese, Chinese or Korean routes keystrokes through an IME: typing
 * `toukyou` produces the reading `とうきょう`, Space offers candidates, arrows
 * walk them, and Enter commits `東京`. Those keys reach the page as well, so a
 * component that acts on Enter or the arrow keys will act while the user is
 * still mid-word.
 *
 * `KeyboardEvent.isComposing` covers most of this, but not all of it, so two
 * further signals are tracked here:
 *
 * - WebKit fires `compositionend` *before* the keydown that ended the
 *   composition, leaving `isComposing` already false on the Enter that commits
 *   a conversion. A keydown arriving immediately after `compositionend` is
 *   therefore still treated as part of it, and that window is closed on the
 *   next keyup — Chromium and Gecko order the same three events keydown →
 *   compositionend → keyup, so their window shuts before the user's next
 *   keystroke.
 * - `compositionstart` / `compositionend` also cover engines that leave
 *   `isComposing` unset on a key that is part of a composition, without
 *   reaching for the deprecated `keyCode === 229` test.
 *
 * Composition events bubble, so this can be bound either to the input itself
 * or to a container above it. Either way the listeners are released with the
 * element they are bound to.
 */
export function trackComposition(
  element: HTMLElement,
): (e: KeyboardEvent) => boolean {
  let composing = false;
  let endedAt = Number.NEGATIVE_INFINITY;

  element.addEventListener("compositionstart", () => {
    composing = true;
  });

  element.addEventListener("compositionend", () => {
    composing = false;
    endedAt = performance.now();
  });

  element.addEventListener("keyup", () => {
    endedAt = Number.NEGATIVE_INFINITY;
  });

  // A composition abandoned by leaving the field never gets its
  // `compositionend`, and a stuck flag would swallow keys indefinitely.
  // `focusout` rather than `blur` so this also holds when tracking a container.
  element.addEventListener("focusout", () => {
    composing = false;
  });

  return (e: KeyboardEvent) =>
    e.isComposing ||
    composing ||
    performance.now() - endedAt < COMPOSITION_TAIL_MS;
}
