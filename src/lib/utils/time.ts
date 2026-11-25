export function formatTime(seconds: number) {
  const whole = Number.isFinite(seconds) ? Math.max(0, Math.floor(seconds)) : 0;
  return `${Math.floor(whole / 60)}:${String(whole % 60).padStart(2, "0")}`;
}

const DATE_FORMAT = new Intl.DateTimeFormat(undefined, { dateStyle: "medium", timeStyle: "short" });

export function formatDate(milliseconds: number) {
  return DATE_FORMAT.format(milliseconds);
}
