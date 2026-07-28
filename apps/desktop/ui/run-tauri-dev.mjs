import { spawn } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const uiDir = path.dirname(fileURLToPath(import.meta.url));
const srcTauriDir = path.resolve(uiDir, "../src-tauri");
const tauriCli = path.join(uiDir, "node_modules", "@tauri-apps", "cli", "tauri.js");

const child = spawn(process.execPath, [tauriCli, "dev"], {
  cwd: srcTauriDir,
  stdio: "inherit",
  env: process.env,
});

child.on("exit", (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
    return;
  }
  process.exit(code ?? 1);
});
