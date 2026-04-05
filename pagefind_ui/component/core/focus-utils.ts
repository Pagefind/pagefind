const FOCUSABLE_SELECTOR = "a[href], button, input, [tabindex]";

type FocusableElement = HTMLElement & { disabled?: boolean };

/**
 * Check whether a container has at least one tabbable child element.
 */
function hasTabbableChild(container: Element): boolean {
  const elements = container.querySelectorAll<HTMLElement>(FOCUSABLE_SELECTOR);
  for (const el of elements) {
    if (el.tabIndex < 0) continue;
    if ((el as FocusableElement).disabled) continue;
    if (el.hasAttribute("hidden")) continue;
    if (window.getComputedStyle(el).display === "none") continue;
    return true;
  }
  return false;
}

/**
 * Given registered components and a starting element, find which component
 * contains the next tabbable element in tab order.
 */
export function findNextComponentInTabOrder(
  fromElement: Element,
  components: HTMLElement[],
): HTMLElement | null {
  let closest: HTMLElement | null = null;

  for (const component of components) {
    // Skip components that contain the current element
    if (component.contains(fromElement)) continue;

    const pos = fromElement.compareDocumentPosition(component);
    // Component must follow fromElement in document order
    if (!(pos & Node.DOCUMENT_POSITION_FOLLOWING)) continue;

    if (!hasTabbableChild(component)) continue;

    if (
      closest === null ||
      component.compareDocumentPosition(closest) &
        Node.DOCUMENT_POSITION_FOLLOWING
    ) {
      closest = component;
    }
  }

  return closest;
}

/**
 * Given registered components and a starting element, find which component
 * contains the previous tabbable element in tab order.
 */
export function findPreviousComponentInTabOrder(
  fromElement: Element,
  components: HTMLElement[],
): HTMLElement | null {
  let closest: HTMLElement | null = null;

  for (const component of components) {
    // Skip components that contain the current element
    if (component.contains(fromElement)) continue;

    const pos = fromElement.compareDocumentPosition(component);
    // Component must precede fromElement in document order
    if (!(pos & Node.DOCUMENT_POSITION_PRECEDING)) continue;

    if (!hasTabbableChild(component)) continue;

    if (
      closest === null ||
      component.compareDocumentPosition(closest) &
        Node.DOCUMENT_POSITION_PRECEDING
    ) {
      closest = component;
    }
  }

  return closest;
}
