# Third-party software and models

VoxMelody is licensed under GPL-3.0; see `LICENSE`. Source: https://github.com/akhil272/voxmelody

## Included in the app

| Component | Licence | Source |
| --- | --- | --- |
| sherpa-onnx 1.13.8 | Apache-2.0 | https://github.com/k2-fsa/sherpa-onnx/tree/v1.13.8 |
| eSpeak NG | GPL-3.0-or-later | https://github.com/csukuangfj/espeak-ng/tree/ed530aa113046142eb5115cf2fc9157854d0ffe1 |
| piper-phonemize | MIT | https://github.com/csukuangfj/piper-phonemize/tree/f3ff95afc03640bc1399e113e83361192a2fafb4 |
| ONNX Runtime | MIT | https://github.com/microsoft/onnxruntime |
| acestep.cpp | MIT | https://github.com/ServeurpersoCom/acestep.cpp/tree/973517be024ff8b6bbd50e0adc9e308ab2ed71b3 |
| ggml | MIT | https://github.com/ServeurpersoCom/ggml/tree/ef4555b7aede4467a5de803b103615e0a5f30dd8 |
| yyjson | MIT | https://github.com/ibireme/yyjson |
| Tauri | MIT or Apache-2.0 | https://github.com/tauri-apps/tauri |
| Svelte | MIT | https://github.com/sveltejs/svelte |
| shadcn-svelte, Bits UI | MIT | https://github.com/huntabyte/shadcn-svelte |
| Lucide | ISC | https://github.com/lucide-icons/lucide |
| Inter, Space Grotesk | OFL-1.1 | https://github.com/rsms/inter, https://github.com/floriankarsten/space-grotesk |

The voice engine (`voxmelody-voice`) statically links sherpa-onnx with eSpeak NG. Its complete source is in `engines/voice` and at the links above.

## Downloaded by the user

| Model | Licence | Source |
| --- | --- | --- |
| Kokoro-82M | Apache-2.0 | https://huggingface.co/hexgrad/Kokoro-82M |
| eSpeak NG language data | GPL-3.0-or-later | https://github.com/espeak-ng/espeak-ng |
| Qwen3-Embedding-0.6B | Apache-2.0 | https://huggingface.co/Qwen/Qwen3-Embedding-0.6B |
| ACE-Step 1.5 | MIT | https://huggingface.co/ACE-Step/Ace-Step1.5 |
| Kitten TTS 0.8 (Nano, Micro, Mini) | Apache-2.0 | https://huggingface.co/KittenML/kitten-tts-mini-0.8 |
| Supertonic 3 | OpenRAIL-M, with use restrictions | https://huggingface.co/Supertone/supertonic-3/blob/main/LICENSE |
| ACE-Step 1.5 SFT and XL Turbo | MIT | https://huggingface.co/ACE-Step |
