import { onMount } from "solid-js";
import { Button } from "~/components/ui/button";

export default function Home() {
  onMount(async () => {});
  return (
    <main class="text-center mx-auto text-gray-700 p-4">
      <Button>Test</Button>
    </main>
  );
}
