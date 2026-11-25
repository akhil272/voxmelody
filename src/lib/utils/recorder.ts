import { MAX_RECORDING_SECONDS } from "#lib/constants.js";

const TICK_MS = 200;

/** Records the microphone until stopped or full. */
export class Recorder {
  #active: { recorder: MediaRecorder; cancelled: boolean } | null = null;
  /** Changes on cancel, so a late permission answer starts nothing. */
  #attempt = 0;
  #onTick: (seconds: number) => void;

  constructor(onTick: (seconds: number) => void) {
    this.#onTick = onTick;
  }

  /**
   * Resolves to true once recording. Calls `onFinish` with the recording, or
   * with nothing when cancelled. Throws a clone error code.
   */
  async start(onFinish: (audio: Blob | null) => void) {
    const attempt = ++this.#attempt;
    const stream = await this.#open();
    const release = () => stream.getTracks().forEach((track) => track.stop());
    if (attempt !== this.#attempt) {
      release();
      return false;
    }
    const recorder = new MediaRecorder(stream);
    const active = { recorder, cancelled: false };
    const parts: Blob[] = [];
    const started = performance.now();
    const timer = setInterval(() => {
      const seconds = (performance.now() - started) / 1000;
      this.#onTick(Math.min(seconds, MAX_RECORDING_SECONDS));
      if (seconds >= MAX_RECORDING_SECONDS) this.stop();
    }, TICK_MS);
    recorder.ondataavailable = ({ data }) => parts.push(data);
    recorder.onstop = () => {
      clearInterval(timer);
      release();
      if (this.#active === active) this.#active = null;
      onFinish(active.cancelled ? null : new Blob(parts, { type: recorder.mimeType }));
    };
    this.#active = active;
    recorder.start();
    return true;
  }

  stop() {
    this.#active?.recorder.stop();
    this.#active = null;
  }

  cancel() {
    this.#attempt++;
    if (this.#active) this.#active.cancelled = true;
    this.stop();
  }

  async #open() {
    try {
      // Processing made for calls changes how a voice sounds.
      const audio = { echoCancellation: false, noiseSuppression: false, autoGainControl: false };
      return await navigator.mediaDevices.getUserMedia({ audio });
    } catch {
      throw new Error("microphone");
    }
  }
}
