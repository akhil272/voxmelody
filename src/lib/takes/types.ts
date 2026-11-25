export interface Take {
  id: number;
  durationMs: number;
  peaks: number[];
}

/** A take that is ready to play. */
export interface Clip extends Take {
  url: string;
  title: string;
}
