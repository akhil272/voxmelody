export type VoiceLanguage = "en-us" | "en-gb" | "es" | "fr" | "hi" | "it" | "pt-br";
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
  language: VoiceLanguage;
}

export interface SpeechRequest {
  text: string;
  voice: string;
  speed: number;
  preview: boolean;
}
