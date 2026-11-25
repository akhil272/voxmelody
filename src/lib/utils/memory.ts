/** Firmware reserves some memory, so allow 10% below the recommendation. */
export function hasEnoughMemory(installed: number, recommended: number) {
  return installed >= recommended * 0.9;
}
