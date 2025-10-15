import { Copy } from "lucide-react";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { copyToClipboard, toastCL } from "@/lib/utils";
import { GameStatus, useGameSessionEvents } from "@/providers/hooks";

const GameSessionMonitor = () => {
  const { gameStatus, logsText, resetGameSessionMonitor } =
    useGameSessionEvents();

  return (
    <Dialog
      open={gameStatus === GameStatus.CRASHED}
      onOpenChange={resetGameSessionMonitor}
    >
      <DialogContent className="max-w-4xl">
        <DialogHeader>
          <DialogTitle>Game exited unexpectedly</DialogTitle>
          <DialogDescription>
            The game may have crashed or exited with an error. Here are the
            logs:
          </DialogDescription>
        </DialogHeader>
        <div className="max-h-96 overflow-y-auto bg-muted p-4 rounded-md">
          <pre className="text-sm">{logsText}</pre>
        </div>
        <DialogFooter>
          <Button
            onClick={() => {
              copyToClipboard(logsText)
                .then(() => {
                  toastCL("success", "Logs copied to clipboard");
                })
                .catch((error) => {
                  toastCL("error", "Error copying logs", error);
                });
            }}
            variant={"ghost"}
          >
            <Copy />
            Copy Logs
          </Button>
          <DialogClose asChild>
            <Button>Close</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default GameSessionMonitor;
