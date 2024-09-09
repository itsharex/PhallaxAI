import { createSignal } from "solid-js";
import { createStore } from "solid-js/store";
import type { State } from "./types";

export const [state, setState] = createStore<State>({
  model: "",
  prompt: "",
  response: "",
  assistant: null,
  config: null,
});

export const [error, setError] = createSignal<string | null>(null);
