import ChatWindow from "~/components/chat/window";
import ChatInput from "~/components/chat/input";
import Error from "~/components/error/error";

export default function Home() {
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
