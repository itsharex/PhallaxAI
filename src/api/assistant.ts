import { invoke } from "@tauri-apps/api/core";
import type { Config, Assistant } from "../types";

export async function initAssistant(
  assistant: Assistant,
  config: Config,
): Promise<void> {
  console.log("initAssistant", assistant, config);
  await invoke("init_ai", {
    assistant: assistant,
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
