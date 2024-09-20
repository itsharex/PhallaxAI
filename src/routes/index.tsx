import { onMount } from "solid-js";

import ChatWindow from "~/components/chat/window";
import ChatInput from "~/components/chat/input";
import Error from "~/components/error/error";
import { connectToDatabase } from "~/api/database";

export default function Home() {
  onMount(async () => {
    await connectToDatabase();
  });
  return (
    <main class="flex flex-col h-screen">
      <div class="flex flex-col h-full">
        <ChatWindow />
        <ChatInput />
      </div>
      <Error />
    </main>
  );
}
