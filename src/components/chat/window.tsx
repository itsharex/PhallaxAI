import { createSignal, createEffect, Index } from "solid-js";
import { state } from "~/state";
import type { ChatHistory } from "~/types";
import UserMessage from "~/components/messages/user";
import BotMessage from "~/components/messages/bot";

export default function ChatWindow() {
  const [messages, setMessages] = createSignal<ChatHistory>([]);

  createEffect(() => {
    const userMessage = state.prompt;
    const assistantMessage = state.response;

    if (userMessage) {
      setMessages((prev) => [...prev, { role: "user", content: userMessage }]);
    } else if (assistantMessage) {
      setMessages((prev) => [
        ...prev,
        { role: "assistant", content: assistantMessage },
      ]);
    }
  });

  return (
    <div class="w-full flex-grow overflow-y-auto">
      <div class="flex flex-col">
        <Index each={messages()} fallback={<div class="flex-grow" />}>
          {(message) =>
            (() => {
              switch (message().role) {
                case "user":
                  return <UserMessage message={message().content} />;
                case "assistant":
                  return <BotMessage message={message().content} />;
                default:
                  return null;
              }
            })()
          }
        </Index>
      </div>
    </div>
  );
}
