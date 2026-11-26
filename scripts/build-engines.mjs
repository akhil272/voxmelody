// Builds engine processes and places them where Tauri bundles them.
// The music engine needs CMake, Ninja and, on Windows, Visual Studio and VULKAN_SDK.
import { execFileSync, execSync } from "node:child_process";
import { copyFileSync, mkdirSync, readdirSync, rmSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("..", import.meta.url));
const triple = /host: (\S+)/.exec(execFileSync("rustc", ["-vV"], { encoding: "utf8" }))[1];
const windows = triple.includes("windows");
const extension = windows ? ".exe" : "";
const binaries = join(root, "src-tauri/binaries");
const libraries = join(binaries, "music");

function place(dir, name) {
  copyFileSync(join(dir, name + extension), join(binaries, `${name}-${triple}${extension}`));
}

function buildVoice() {
  const cwd = join(root, "engines/voice");
  execFileSync("cargo", ["build", "--release", "--locked"], { cwd, stdio: "inherit" });
  place(join(cwd, "target/release"), "voxmelody-voice");
}

/** Visual Studio provides the compiler, CMake and Ninja. */
function compilerSetup() {
  const vswhere = join(process.env["ProgramFiles(x86)"], "Microsoft Visual Studio/Installer/vswhere.exe");
  const query = ["-latest", "-products", "*", "-property", "installationPath"];
  const studio = execFileSync(vswhere, query, { encoding: "utf8" }).trim();
  return `call "${join(studio, "VC/Auxiliary/Build/vcvars64.bat")}" >nul && `;
}

function buildMusic() {
  const cwd = join(root, "engines/music");
  const sdk = process.env.VULKAN_SDK;
  if (windows && !sdk) throw new Error("Set VULKAN_SDK to build the music engine.");
  const setup = windows ? compilerSetup() : "";
  const prefix = windows ? ` -DCMAKE_PREFIX_PATH="${sdk.replaceAll("\\", "/")}"` : "";
  const configure = `cmake -S . -B .build -G Ninja -DCMAKE_BUILD_TYPE=Release${prefix}`;
  const env = { ...process.env };
  if (windows) env.PATH = `${join(sdk, "bin")};${env.PATH}`;
  // One shell, so both steps see the compiler environment.
  execSync(`${setup}${configure} && cmake --build .build`, { cwd, env, stdio: "inherit" });

  const output = join(cwd, ".build/bin");
  place(output, "voxmelody-music");
  if (!windows) return;
  rmSync(libraries, { recursive: true, force: true });
  mkdirSync(libraries, { recursive: true });
  const names = readdirSync(output).filter((name) => /^ggml.*\.dll$/.test(name));
  for (const name of names) copyFileSync(join(output, name), join(libraries, name));
}

mkdirSync(binaries, { recursive: true });
buildVoice();
buildMusic();
