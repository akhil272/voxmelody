import { cn, waveformHeights } from "@/lib/utils";

interface WaveformProps {
  tone: "voice" | "music";
  bars?: number;
}

export function Waveform({ tone, bars = 25 }: WaveformProps) {
  return (
    <div className="flex h-14 shrink-0 items-center gap-[3px]" aria-hidden>
      {waveformHeights(bars).map((height, index) => (
        <span
          key={index}
          className={cn("w-[2px] rounded-full", tone === "voice" ? "bg-teal-bright" : "bg-secondary")}
          style={{ height: `${height * 100}%` }}
        />
      ))}
    </div>
  );
}
