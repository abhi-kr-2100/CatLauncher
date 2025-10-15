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
import { UPDATE_LINK } from "@/lib/constants";
import { openLink } from "@/lib/utils";
import { AutoUpdateStatus, useAutoUpdateEvents } from "@/providers/hooks";

const AutoUpdateNotifier = () => {
  const { autoUpdateStatus, resetAutoUpdateStatus } = useAutoUpdateEvents();

  return (
    <Dialog
      open={autoUpdateStatus === AutoUpdateStatus.FAILURE}
      onOpenChange={resetAutoUpdateStatus}
    >
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
                resetAutoUpdateStatus();
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
