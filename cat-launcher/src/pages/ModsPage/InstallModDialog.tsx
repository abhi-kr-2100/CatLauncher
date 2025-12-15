import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { useGetModActivity } from "./hooks";
import { GameVariant } from "@/generated-types/GameVariant";

interface InstallModDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
  modId: string;
  variant: GameVariant;
}

export default function InstallModDialog({
  open,
  onOpenChange,
  onConfirm,
  modId,
  variant,
}: InstallModDialogProps) {
  const { activity, isLoading, isError } = useGetModActivity(
    modId,
    variant,
  );

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Install Mod</DialogTitle>
          <DialogDescription>
            Are you sure you want to install this mod?
          </DialogDescription>
        </DialogHeader>
        <div>
          {isLoading && <p>Loading activity...</p>}
          {isError && <p>Failed to load activity.</p>}
          {activity && (
            <p>
              Last updated: {new Date(activity).toLocaleDateString()}
            </p>
          )}
        </div>
        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button onClick={onConfirm}>Confirm</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
