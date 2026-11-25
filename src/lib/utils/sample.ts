import { MAX_SAMPLE_BYTES, SAMPLE_RATE } from "#lib/constants.js";

/** Any recording the system can play, as mono. Throws a clone error code. */
export async function decodeSample(audio: Blob): Promise<Float32Array<ArrayBuffer>> {
  if (audio.size > MAX_SAMPLE_BYTES) throw new Error("audio");
  // Decoding converts to the rate of the context.
  const context = new OfflineAudioContext(1, 1, SAMPLE_RATE);
  const decoded = await context.decodeAudioData(await audio.arrayBuffer()).catch(() => {
    throw new Error("audio");
  });
  const mono = new Float32Array(decoded.length);
  for (let channel = 0; channel < decoded.numberOfChannels; channel++) {
    const data = decoded.getChannelData(channel);
    for (let index = 0; index < mono.length; index++) mono[index] += data[index] / decoded.numberOfChannels;
  }
  return mono;
}
