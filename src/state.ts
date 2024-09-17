import { createSignal } from "solid-js";
import { createStore } from "solid-js/store";
import type { State } from "./types";

export const [state, setState] = createStore<State>({
  model: "",
  prompt: "",
  response: "",
  assistant: {
    name: "wizard-vicuna-uncensored:13B",
    instructions: "You are a helpful assistant.",
    config_id: 1,
    created_at: "",
    updated_at: "",
  },
  config: {
    num_ctx: 2048,
    temperature: 0.8,
    frequency_penalty: 0.0,
    presence_penalty: 0.0,
  },
});

export const [error, setError] = createSignal<string | null>(null);
