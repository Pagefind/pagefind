interface NavigatorUAData {
  platform?: string;
}

export interface KeyBinding {
  usesCtrl: boolean; // Platform-aware: Ctrl on Windows/Linux, Cmd on Mac
  key: string;
}

export function detectMac(): boolean {
  try {
    const uaData = (
      navigator as Navigator & { userAgentData?: NavigatorUAData }
    ).userAgentData;
    if (uaData?.platform) {
      return uaData.platform.toLowerCase().includes("mac");
    }
  } catch {
    // Fall back to userAgent check
  }
  return /mac/i.test(navigator.userAgent);
}

export function parseKeyBinding(bindingStr: string): KeyBinding {
  const parts = bindingStr.toLowerCase().split("+");
  const usesCtrl = parts.includes("ctrl") || parts.includes("cmd");

  // Get the key (anything that's not a modifier)
  let key = parts.find((p) => p !== "ctrl" && p !== "cmd") || "";

  // Handle special key names
  if (key === "slash") {
    key = "/";
  }

  return { usesCtrl, key };
}

export function keyBindingMatches(
  binding: KeyBinding,
  event: KeyboardEvent,
): boolean {
  const isMac = detectMac();
  const keyMatches = event.key.toLowerCase() === binding.key;
  const modifierMatches = binding.usesCtrl
    ? isMac
      ? event.metaKey
      : event.ctrlKey
    : !(event.ctrlKey || event.metaKey);

  return keyMatches && modifierMatches && !event.shiftKey && !event.altKey;
}

export function getShortcutDisplay(
  binding: KeyBinding,
  isMac: boolean,
): {
  keys: string[];
  aria: string;
} {
  const keys: string[] = [];

  if (binding.usesCtrl) {
    keys.push(isMac ? "⌘" : "Ctrl");
  }

  // Format the key
  const keyDisplay = binding.key === "/" ? "/" : binding.key.toUpperCase();
  keys.push(keyDisplay);

  // ARIA format
  const ariaModifier = binding.usesCtrl ? (isMac ? "Meta" : "Control") : "";
  const aria = ariaModifier ? `${ariaModifier}+${keyDisplay}` : keyDisplay;

  return { keys, aria };
}
