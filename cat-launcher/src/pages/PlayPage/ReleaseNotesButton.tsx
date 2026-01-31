import { FileText } from "lucide-react";
import { useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

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
import { openLink } from "@/lib/utils";
import { useReleaseNotesRange } from "./hooks";
import ReleaseSelectionColumn from "./ReleaseSelectionColumn";
import { GameVariant } from "@/generated-types/GameVariant";

interface ReleaseNotesButtonProps {
  variant: GameVariant;
}

export default function ReleaseNotesButton({
  variant,
}: ReleaseNotesButtonProps) {
  const [isOpen, setIsOpen] = useState(false);

  const {
    fromId,
    setFromId,
    toId,
    setToId,
    combinedNotes,
    isLoading,
    isReversed,
    handleSwap,
    targetVersions,
  } = useReleaseNotesRange(variant);

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">
          <FileText className="h-4 w-4" />
          What's new?
        </Button>
      </DialogTrigger>
      <DialogContent className="flex max-h-[80vh] sm:max-w-5xl flex-col">
        <DialogHeader>
          <DialogTitle>Release Notes</DialogTitle>
          <DialogDescription className="sr-only">
            Release notes comparison
          </DialogDescription>
        </DialogHeader>

        <div className="flex gap-6 p-1">
          <ReleaseSelectionColumn
            label="From"
            variant={variant}
            selectedReleaseId={fromId}
            onSelect={setFromId}
            targetVersions={targetVersions}
          />
          <ReleaseSelectionColumn
            label="To"
            variant={variant}
            selectedReleaseId={toId}
            onSelect={setToId}
            targetVersions={targetVersions}
          />
        </div>

        <div className="prose prose-sm dark:prose-invert flex-1 overflow-y-auto max-w-none px-1">
          {isReversed ? (
            <div className="flex flex-col items-center justify-center h-full gap-4 text-center">
              <p className="text-muted-foreground">
                The "From" version is newer than the "To" version.
              </p>
              <Button onClick={handleSwap} variant="outline">
                Swap versions
              </Button>
            </div>
          ) : isLoading ? (
            "Loading..."
          ) : (
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              components={{
                a: ({ node: _node, ...props }) => (
                  <a
                    {...props}
                    onClick={(e) => {
                      if (props.href) {
                        e.preventDefault();
                        openLink(props.href);
                      }
                    }}
                  />
                ),
              }}
            >
              {combinedNotes ?? ""}
            </ReactMarkdown>
          )}
        </div>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Close</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
