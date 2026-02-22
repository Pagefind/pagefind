interface NavigatorUAData {
  platform?: string;
}

export interface KeyBinding {
  mod: boolean;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
  meta: boolean;
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
  const binding: KeyBinding = {
    mod: false,
    ctrl: false,
    shift: false,
    alt: false,
    meta: false,
    key: "",
  };

  for (const part of parts) {
    switch (part) {
      case "mod":
        binding.mod = true;
        break;
      case "ctrl":
        binding.ctrl = true;
        break;
      case "shift":
        binding.shift = true;
        break;
      case "alt":
        binding.alt = true;
        break;
      case "meta":
      case "cmd":
      case "command":
        binding.meta = true;
        break;
      default:
        binding.key = part;
    }
  }

  return binding;
}

export function keyBindingMatches(
  binding: KeyBinding,
  event: KeyboardEvent,
): boolean {
  const isMac = detectMac();
  const keyMatches = event.key.toLowerCase() === binding.key;

  // mod is platform-aware: ctrl on Windows/Linux, cmd on Mac
  const modCtrl = binding.mod ? !isMac : binding.ctrl;
  const modMeta = binding.mod ? isMac : binding.meta;

  const ctrlMatch = modCtrl ? event.ctrlKey : !event.ctrlKey;
  const metaMatch = modMeta ? event.metaKey : !event.metaKey;
  const shiftMatch = binding.shift ? event.shiftKey : !event.shiftKey;
  const altMatch = binding.alt ? event.altKey : !event.altKey;

  return keyMatches && ctrlMatch && metaMatch && shiftMatch && altMatch;
}

export function getShortcutDisplay(binding: KeyBinding): {
  keys: string[];
  aria: string;
} {
  const isMac = detectMac();
  const keys: string[] = [];
  const ariaParts: string[] = [];

  if (binding.mod) {
    keys.push(isMac ? "⌘" : "Ctrl");
    ariaParts.push(isMac ? "Meta" : "Control");
  }
  if (binding.meta) {
    keys.push(isMac ? "⌘" : "⊞");
    ariaParts.push("Meta");
  }
  if (binding.ctrl) {
    keys.push("Ctrl");
    ariaParts.push("Control");
  }
  if (binding.shift) {
    keys.push("Shift");
    ariaParts.push("Shift");
  }
  if (binding.alt) {
    keys.push("Alt");
    ariaParts.push("Alt");
  }

  const keyDisplay = binding.key.toUpperCase();
  keys.push(keyDisplay);
  ariaParts.push(keyDisplay);

  return { keys, aria: ariaParts.join("+") };
}
