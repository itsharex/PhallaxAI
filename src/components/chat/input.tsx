import { Button } from "~/components/ui/button";
import { TextArea } from "~/components/ui/textarea";
import { TextFieldRoot } from "~/components/ui/textfield";

export default function ChatInput() {
  return (
    <div class="w-full flex-none">
      <TextFieldRoot class="w-full">
        <TextArea
          class="resize-none"
          cols={50}
          rows={1}
          autoResize={true}
          placeholder="Message AI"
        />
      </TextFieldRoot>
      <Button class="w-full">Send</Button>
    </div>
  );
}
