import { MicIcon, MusicIcon } from "lucide-react";
import { waveBand, wavePath } from "@/lib/utils";

const WAVE_LINES = 14;
const WAVE_SHAPE = { width: 560, baseline: 145, cycles: 1.5 };

export function HeroWave() {
  return (
    <div className="pointer-events-none absolute inset-y-0 right-0 w-3/5" aria-hidden>
      <svg viewBox="0 0 560 270" preserveAspectRatio="none" className="size-full" fill="none">
        <defs>
          <linearGradient id="vm-wave" gradientUnits="userSpaceOnUse" x1="0" x2="560">
            <stop stopColor="var(--teal-bright)" stopOpacity="0" />
            <stop offset="0.35" stopColor="var(--teal-bright)" />
            <stop offset="1" stopColor="var(--secondary)" />
          </linearGradient>
        </defs>
        {Array.from({ length: WAVE_LINES }, (_, index) => (
          <path
            key={index}
            d={wavePath({ ...WAVE_SHAPE, amplitude: 88 - index * 5, phase: 2.6 + index * 0.06 })}
            stroke="url(#vm-wave)"
            strokeOpacity={0.6 - index * 0.03}
          />
        ))}
        <path
          d={waveBand({ ...WAVE_SHAPE, amplitude: 88, phase: 2.6 }, { ...WAVE_SHAPE, amplitude: 45, phase: 3.9 })}
          fill="url(#vm-wave)"
          fillOpacity="0.14"
        />
      </svg>
      <span className="absolute top-[18%] left-[48%] grid size-13 place-items-center rounded-full bg-card/90 text-primary shadow-md">
        <MusicIcon className="size-6" />
      </span>
      <span className="absolute top-[40%] left-[66%] grid size-17 place-items-center rounded-full bg-card/90 text-secondary shadow-md">
        <MicIcon className="size-8" />
      </span>
    </div>
  );
}
