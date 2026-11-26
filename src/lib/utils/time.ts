export function formatTime(seconds: number) {
  const whole = Number.isFinite(seconds) ? Math.max(0, Math.floor(seconds)) : 0;
  return `${Math.floor(whole / 60)}:${String(whole % 60).padStart(2, "0")}`;
}

const DATE_FORMAT = new Intl.DateTimeFormat(undefined, { dateStyle: "medium", timeStyle: "short" });

const MONTH_FORMAT = new Intl.DateTimeFormat(undefined, { month: "short", year: "numeric", timeZone: "UTC" });

/** From YYYY-MM-DD. */
export function formatMonth(date: string) {
  return MONTH_FORMAT.format(new Date(date));
}

export function formatDate(milliseconds: number) {
  return DATE_FORMAT.format(milliseconds);
}
