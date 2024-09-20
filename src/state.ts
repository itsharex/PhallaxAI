import { createSignal } from "solid-js";
import { createStore } from "solid-js/store";
import type { State } from "./types";

export const [state, setState] = createStore<State>({
  model: "wizard-vicuna-uncensored",
  prompt: "",
  response: "",
  assistant: {
    id: 0,
    name: "wizard-vicuna-uncensored",
    instructions: "You are a helpful assistant.",
    config_id: 0,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
  },
  config: {
    id: 0,
    num_ctx: 2048,
    temperature: 0.8,
    frequency_penalty: 0.0,
    presence_penalty: 0.0,
  },
});

export const [error, setError] = createSignal<string | null>(null);
