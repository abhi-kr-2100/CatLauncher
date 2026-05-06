import React, { useEffect, useState } from "react";

import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";
import { confirmQuit, listenToQuitRequested } from "@/lib/commands";
import { setupEventListener } from "@/lib/utils";
import { useAppSelector } from "@/store/hooks";

/**
 * A provider component that intercepts the application's quit request.
 * If a game is currently running, it displays a confirmation dialog to warn the user
 * about the consequences of quitting (e.g., play time not recorded, logs not saved).
 * If no game is running, it allows the application to quit immediately.
 *
 * @param props - The component props containing children to be rendered.
 * @returns A React component that wraps its children and handles quit confirmation.
 */
export default function QuitConfirmationProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [quitDialogOpen, setQuitDialogOpen] = useState(false);
  const isCurrentlyPlaying = useAppSelector(
    (state) => state.gameSession.currentlyPlaying != null,
  );

  useEffect(() => {
    const quitHandler = () => {
      if (isCurrentlyPlaying) {
        setQuitDialogOpen(true);
      } else {
        confirmQuit();
      }
    };

    const cleanup = setupEventListener(
      (handler) => listenToQuitRequested(() => handler(undefined)),
      quitHandler,
      "Error listening to quit request.",
    );

    return cleanup;
  }, [isCurrentlyPlaying]);

  return (
    <>
      {children}
      <ConfirmationDialog
        open={quitDialogOpen}
        onOpenChange={setQuitDialogOpen}
        onConfirm={confirmQuit}
        title="Quit CatLauncher?"
        description="If you quit CatLauncher, play time won't be recorded. Additionally, CatLauncher won't be able to save logs in case your game crashes. It's not recommended to quit CatLauncher while a game is running."
        confirmText="Quit"
        cancelText="Cancel"
      />
    </>
  );
}
