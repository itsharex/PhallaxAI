import { onMount } from "solid-js";
import { Button } from "~/components/ui/button";
import Database from "@tauri-apps/plugin-sql";
import { BaseDirectory } from "@tauri-apps/plugin-fs";

export default function Home() {
  onMount(async () => {
    const db = await Database.load("sqlite:phallax.db");
    console.log(db.path);
    console.log(BaseDirectory.Home);
  });
  return (
    <main class="text-center mx-auto text-gray-700 p-4">
      <Button>Test</Button>
    </main>
  );
}
