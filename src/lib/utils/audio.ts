export function toAudioUrl(data: ArrayBuffer) {
  return URL.createObjectURL(new Blob([data], { type: "audio/wav" }));
}
