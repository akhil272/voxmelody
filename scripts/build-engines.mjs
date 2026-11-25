// Builds engine processes and places them where Tauri bundles sidecars.
import { execFileSync } from "node:child_process";
import { copyFileSync, mkdirSync } from "node:fs";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("..", import.meta.url));
const triple = /host: (\S+)/.exec(execFileSync("rustc", ["-vV"], { encoding: "utf8" }))[1];
const extension = triple.includes("windows") ? ".exe" : "";
const binaries = `${root}src-tauri/binaries`;

mkdirSync(binaries, { recursive: true });
execFileSync("cargo", ["build", "--release", "--locked"], { cwd: `${root}engines/voice`, stdio: "inherit" });
copyFileSync(
  `${root}engines/voice/target/release/voxmelody-voice${extension}`,
  `${binaries}/voxmelody-voice-${triple}${extension}`,
);
