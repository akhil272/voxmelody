export function isMacPlatform() {
  return navigator.userAgent.includes("Macintosh");
}

/** Subscription for values that never change after load. */
export function subscribeNever() {
  return () => undefined;
}
