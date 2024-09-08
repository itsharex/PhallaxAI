import { createSignal } from "solid-js";

interface UserMessageProps {
  message: string;
}

export default function UserMessage(props: UserMessageProps) {
  const [message, setMessage] = createSignal(props.message);
  return (
    <div class="bg-neutral-300 dark:text-white dark:bg-neutral-600 justify-center p-5">
      <div class="mx-auto max-w-[50%]">
        <p>{message()}</p>
      </div>
    </div>
  );
}
