const FOCUSABLE_SELECTOR = "a[href], button, input, [tabindex]";

type FocusableElement = HTMLElement & { disabled?: boolean };

/**
 * Get all tabbable elements in tab order.
 */
export function getTabbablesInOrder(
  container: Document | Element = document,
): HTMLElement[] {
  const elements = Array.from(
    container.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR),
  );

  const tabbable = elements.filter((el): el is FocusableElement => {
    if (el.tabIndex < 0) return false;
    if ((el as FocusableElement).disabled) return false;
    if (el.hasAttribute("hidden")) return false;
    if (window.getComputedStyle(el).display === "none") return false;
    return true;
  });

  const withPositiveTabIndex: HTMLElement[] = [];
  const withZeroTabIndex: HTMLElement[] = [];

  for (const el of tabbable) {
    if (el.tabIndex > 0) {
      withPositiveTabIndex.push(el);
    } else {
      withZeroTabIndex.push(el);
    }
  }

  withPositiveTabIndex.sort((a, b) => a.tabIndex - b.tabIndex);
  return [...withPositiveTabIndex, ...withZeroTabIndex];
}

/**
 * Given registered components and a starting element, find which component
 * contains the next tabbable element in tab order.
 */
export function findNextComponentInTabOrder(
  fromElement: Element,
  components: HTMLElement[],
): HTMLElement | null {
  const tabbables = getTabbablesInOrder();
  const currentIndex = tabbables.indexOf(fromElement as HTMLElement);
  if (currentIndex === -1) return null;

  const componentsWithTabPos = components
    .map((component) => {
      const firstTabbable = tabbables.find((t) => component.contains(t));
      return {
        component,
        tabPos: firstTabbable ? tabbables.indexOf(firstTabbable) : -1,
      };
    })
    .filter((c) => c.tabPos > currentIndex)
    .sort((a, b) => a.tabPos - b.tabPos);

  return componentsWithTabPos[0]?.component || null;
}

/**
 * Given registered components and a starting element, find which component
 * contains the previous tabbable element in tab order.
 */
export function findPreviousComponentInTabOrder(
  fromElement: Element,
  components: HTMLElement[],
): HTMLElement | null {
  const tabbables = getTabbablesInOrder();
  const currentIndex = tabbables.indexOf(fromElement as HTMLElement);
  if (currentIndex === -1) return null;

  const componentsWithTabPos = components
    .map((component) => {
      const componentTabbables = tabbables.filter((t) => component.contains(t));
      const lastTabbable = componentTabbables[componentTabbables.length - 1];
      return {
        component,
        tabPos: lastTabbable ? tabbables.indexOf(lastTabbable) : -1,
      };
    })
    .filter((c) => c.tabPos >= 0 && c.tabPos < currentIndex)
    .sort((a, b) => b.tabPos - a.tabPos);

  return componentsWithTabPos[0]?.component || null;
}
