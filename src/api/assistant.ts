import { invoke } from "@tauri-apps/api/core";
import type { Config, Assistant } from "../types";

export async function initAssistant(
  asssistant: Assistant,
  config: Config,
): Promise<void> {
  await invoke("init_ai", {
    assistant: asssistant,
    config: config,
  });
}

export async function completion(
  model: string,
  prompt: string,
  assistant: Assistant,
  config: Config,
): Promise<string> {
  return await invoke("completion", {
    model: model,
    prompt: prompt,
    assistant: assistant,
    config: config,
  });
}
