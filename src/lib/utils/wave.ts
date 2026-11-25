interface WaveOptions {
  width: number;
  baseline: number;
  amplitude: number;
  cycles: number;
  phase: number;
}

const WAVE_STEPS = 48;

function wavePoints({ width, baseline, amplitude, cycles, phase }: WaveOptions) {
  return Array.from({ length: WAVE_STEPS + 1 }, (_, step) => {
    const progress = step / WAVE_STEPS;
    const envelope = 0.35 + 0.65 * Math.sin(Math.PI * (0.15 + progress * 0.7));
    const y = baseline + Math.sin(progress * cycles * 2 * Math.PI + phase) * amplitude * envelope;
    return `${(progress * width).toFixed(1)} ${y.toFixed(1)}`;
  });
}

export function wavePath(options: WaveOptions) {
  return `M${wavePoints(options).join("L")}`;
}

/** Closed area between two waves. */
export function waveBand(upper: WaveOptions, lower: WaveOptions) {
  return `${wavePath(upper)}L${wavePoints(lower).reverse().join("L")}Z`;
}

/** Deterministic bar heights from 0.15 to 1, tallest near the middle. */
export function waveformHeights(count: number) {
  return Array.from({ length: count }, (_, index) => {
    const envelope = Math.sin((Math.PI * (index + 0.5)) / count);
    const texture = 0.45 + 0.55 * Math.abs(Math.sin(index * 2.4));
    return Number(Math.max(0.15, envelope * texture).toFixed(3));
  });
}
