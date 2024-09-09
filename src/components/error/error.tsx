import { toaster } from "@kobalte/core";
import {
  Toast,
  ToastContent,
  ToastDescription,
  ToastTitle,
} from "~/components/ui/toast";
import { error } from "~/state";
import { createEffect } from "solid-js";

export default function Error() {
  createEffect(() => {
    if (error()) {
      showToast();
    }
  });

  const showToast = () =>
    toaster.show((props) => (
      <Toast toastId={props.toastId} variant="destructive">
        <ToastContent>
          <ToastTitle>Error</ToastTitle>
          <ToastDescription>{error() || "An error occurred."}</ToastDescription>
        </ToastContent>
      </Toast>
    ));

  return <></>;
}
