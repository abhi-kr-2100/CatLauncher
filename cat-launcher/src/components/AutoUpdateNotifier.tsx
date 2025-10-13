import { useEffect, useState } from "react";

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
import { listenToAutoupdateStatus } from "@/lib/commands";
import { UPDATE_LINK } from "@/lib/constants";
import { openLink, toastCL } from "@/lib/utils";

const AutoUpdateNotifier = () => {
  const [isFailureDialogOpen, setIsFailureDialogOpen] = useState(false);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    listenToAutoupdateStatus((status) => {
      switch (status.type) {
        case "Failure":
          setIsFailureDialogOpen(true);
          break;
      }
    })
      .then((unlistenFn) => {
        unlisten = unlistenFn;
      })
      .catch((error) => {
        toastCL("error", "Error listening to autoupdate status", error);
      });

    return () => {
      unlisten?.();
    };
  }, []);

  return (
    <Dialog open={isFailureDialogOpen} onOpenChange={setIsFailureDialogOpen}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Autoupdate Failed</DialogTitle>
          <DialogDescription>
            Please manually update the app by visiting
            <Button
              className="p-0"
              variant="link"
              onClick={() => {
                openLink(UPDATE_LINK);
                setIsFailureDialogOpen(false);
              }}
            >
              {UPDATE_LINK}
            </Button>
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose asChild>
            <Button>Close</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default AutoUpdateNotifier;
