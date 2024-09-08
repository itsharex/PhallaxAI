import { createSignal } from "solid-js";

interface BotMessageProps {
  message: string;
}

export default function BotMessage(props: BotMessageProps) {
  const [message, setMessage] = createSignal(props.message);
  return (
    <div class="dark:text-white dark:bg-neutral-700 p-5">
      <div class="mx-auto max-w-[50%]">
        <p>{message()}</p>
      </div>
    </div>
  );
}
