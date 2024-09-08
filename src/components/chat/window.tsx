import UserMessage from "~/components/messages/user";
import BotMessage from "~/components/messages/bot";

export default function ChatWindow() {
  return (
    <div class="w-full flex-grow overflow-y-auto">
      <div class="flex flex-col">
        <UserMessage message="Hello" />
        <BotMessage message="Hi, how can I help you?" />
      </div>
    </div>
  );
}
