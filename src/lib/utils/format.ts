const UNITS = ["B", "KB", "MB", "GB", "TB"];

export function formatBytes(bytes: number) {
  const exponent = bytes > 0 ? Math.min(Math.floor(Math.log10(bytes) / 3), UNITS.length - 1) : 0;
  const value = bytes / 1000 ** exponent;
  return `${value.toFixed(exponent > 2 ? 1 : 0)} ${UNITS[exponent]}`;
}

/** Installed memory is sold in binary units, so 16 GiB reads as 16 GB. */
export function formatMemory(bytes: number) {
  return `${Math.round(bytes / 1024 ** 3)} GB`;
}

export function toPercent(part: number, total: number) {
  return total > 0 ? Math.floor((part / total) * 100) : 0;
}
