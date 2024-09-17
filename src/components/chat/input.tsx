import { createEffect, createSignal } from "solid-js";
import { Button } from "~/components/ui/button";
import { TextArea } from "~/components/ui/textarea";
import { TextFieldRoot } from "~/components/ui/textfield";
import { completion, initAssistant } from "~/api/assistant";
import { state, setState, setError } from "~/state";

export default function ChatInput() {
  const [prompt, setPrompt] = createSignal("");
  const [clear, setClear] = createSignal(false);

  createEffect(async () => {
    const userMessage = prompt();
    const assistant = state.assistant;
    const config = state.config;
    if (!assistant || !config) return;
    await initAssistant(assistant, config);
    if (clear() && userMessage && assistant && config) {
      setState({ prompt: userMessage });
      try {
        const assistantMessage = await completion(
          state.model,
          userMessage,
          assistant,
          config,
        );
        setState({ response: assistantMessage });
      } catch (e: any) {
        console.error(e);
        setError(e.message);
      } finally {
        setPrompt("");
        setClear(false);
      }
    }
  });

  return (
    <div class="w-full flex-none">
      <TextFieldRoot class="w-full">
        <TextArea
          class="resize-none"
          cols={50}
          rows={1}
          onChange={(e: any) => setPrompt(e.currentTarget.value)}
          autoResize={true}
          placeholder="Message AI"
        />
      </TextFieldRoot>
      <Button class="w-full" onClick={() => setClear(true)}>
        Send
      </Button>
    </div>
  );
}
