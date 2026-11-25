import { COPY } from "@/lib/constants";
import { cn } from "@/lib/utils";

interface LogoProps {
  className?: string;
  decorative?: boolean;
}

export function Logo({ className, decorative = false }: LogoProps) {
  return (
    <svg
      viewBox="0 0 142 80"
      className={cn("h-5 w-auto", className)}
      fill="none"
      strokeWidth="21"
      strokeLinecap="round"
      strokeLinejoin="round"
      {...(decorative ? { "aria-hidden": true } : { role: "img", "aria-label": COPY.appName })}
    >
      <defs>
        <linearGradient id="vm-teal" x1="0" y1="0" x2="1" y2="1">
          <stop stopColor="var(--teal-bright)" />
          <stop offset="1" stopColor="var(--primary)" />
        </linearGradient>
        <linearGradient id="vm-cobalt" x1="0" y1="1" x2="1" y2="0">
          <stop stopColor="var(--secondary)" />
          <stop offset="1" stopColor="var(--cobalt-bright)" />
        </linearGradient>
      </defs>
      <path d="M13 14 40 62 69 14 90 46" stroke="url(#vm-teal)" />
      <path d="M78 66 105 18 130 54" stroke="url(#vm-cobalt)" />
      <circle cx="76" cy="66" r="13" fill="var(--secondary)" />
    </svg>
  );
}
