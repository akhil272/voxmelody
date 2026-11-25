export type VoiceError =
  | "notInstalled"
  | "noSpeech"
  | "tooLong"
  | "model"
  | "output"
  | "request"
  | "engine"
  | "cancelled";

export interface Voice {
  id: string;
  name: string;
  model: string;
  language: string;
}

export interface SpeechRequest {
  text: string;
  model: string;
  voice: string;
  speed: number;
  preview: boolean;
  pause: number;
  threads: number;
  steps: number;
}
