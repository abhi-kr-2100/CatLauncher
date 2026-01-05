import { FileText } from "lucide-react";
import { useState } from "react";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import type { GameRelease } from "@/generated-types/GameRelease";
import { toastCL } from "@/lib/utils";
import useReleaseNotes from "./hooks/useReleaseNotes";

interface ReleaseNotesButtonProps {
  release: GameRelease | undefined;
}

export default function ReleaseNotesButton({
  release,
}: ReleaseNotesButtonProps) {
  const [isOpen, setIsOpen] = useState(false);

  const { notes, isLoading } = useReleaseNotes(release, (error) => {
    const errorMessage = release
      ? `Failed to fetch release notes for ${release.version}.`
      : "Failed to fetch release notes.";
    toastCL("error", errorMessage, error);
  });

  if (!release) {
    return null;
  }

  const label = "Release Notes";
  const loadingLabel = "Loading...";
  const emptyLabel = "No release notes available.";
  const closeLabel = "Close";

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button variant="outline" title={label}>
          <FileText className="h-4 w-4" />
          {label}
        </Button>
      </DialogTrigger>
      <DialogContent className="flex max-h-[80vh] max-w-3xl flex-col">
        <DialogHeader>
          <DialogTitle>{label}</DialogTitle>
          <DialogDescription className="sr-only">
            Release notes for version {release.version}
          </DialogDescription>
        </DialogHeader>
        <div className="flex-1 overflow-y-auto whitespace-pre-wrap font-mono text-sm">
          {isLoading
            ? loadingLabel
            : notes || release.body || emptyLabel}
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">{closeLabel}</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
