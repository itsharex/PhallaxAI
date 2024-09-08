import ChatWindow from "~/components/chat/window";
import ChatInput from "~/components/chat/input";

export default function Home() {
  return (
    <main class="flex flex-col h-screen">
      <div class="flex flex-col h-full">
        <ChatWindow />
        <ChatInput />
      </div>
    </main>
  );
}
