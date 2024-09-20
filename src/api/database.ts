import { invoke } from "@tauri-apps/api/core";

export async function connectToDatabase(): Promise<void> {
  try {
    await invoke("connect_to_database");
  } catch (error) {
    console.error(error);
  }
}
