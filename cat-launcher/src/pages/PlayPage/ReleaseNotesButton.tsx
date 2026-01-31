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
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import { openLink } from "@/lib/utils";
import { useReleaseNotesRange, QuickSelectKey } from "./hooks";
import ReleaseDropdown from "./ReleaseDropdown";

interface ReleaseNotesButtonProps {
  release: GameRelease;
}

export default function ReleaseNotesButton({
  release,
}: ReleaseNotesButtonProps) {
  const [isOpen, setIsOpen] = useState(false);
  const variant = release.variant;

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
  } = useReleaseNotesRange(variant, release.version);

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
          <div className="flex-1 flex flex-col gap-1">
            <span className="text-sm font-medium">From</span>
            <ReleaseDropdown
              variant={variant}
              selectedReleaseId={fromId}
              setSelectedReleaseId={setFromId}
              hideActiveLabel
            />
            <div className="grid grid-cols-2 gap-2 mt-1">
              {getQuickSelectButtons(variant).map((btn) => {
                const version = targetVersions[btn.key];
                return (
                  <Button
                    key={btn.label}
                    variant="secondary"
                    size="sm"
                    className="h-7 text-xs px-2"
                    disabled={!version}
                    onClick={() => version && setFromId(version)}
                  >
                    {btn.label}
                  </Button>
                );
              })}
            </div>
          </div>
          <div className="flex-1 flex flex-col gap-1">
            <span className="text-sm font-medium">To</span>
            <ReleaseDropdown
              variant={variant}
              selectedReleaseId={toId}
              setSelectedReleaseId={setToId}
              hideActiveLabel
            />
            <div className="grid grid-cols-2 gap-2 mt-1">
              {getQuickSelectButtons(variant).map((btn) => {
                const version = targetVersions[btn.key];
                return (
                  <Button
                    key={btn.label}
                    variant="secondary"
                    size="sm"
                    className="h-7 text-xs px-2"
                    disabled={!version}
                    onClick={() => version && setToId(version)}
                  >
                    {btn.label}
                  </Button>
                );
              })}
            </div>
          </div>
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

function getQuickSelectButtons(
  variant: GameVariant,
): { label: string; key: QuickSelectKey }[] {
  switch (variant) {
    case "DarkDaysAhead":
      return [
        { label: "Active", key: "Active" },
        { label: "Latest Stable", key: "Stable" },
        {
          label: "Latest Release Candidate",
          key: "ReleaseCandidate",
        },
        { label: "Latest Experimental", key: "Experimental" },
      ];
    case "BrightNights":
      return [
        { label: "Active", key: "Active" },
        { label: "Latest Stable", key: "Stable" },
        { label: "Latest Experimental", key: "Experimental" },
      ];
    case "TheLastGeneration":
      return [
        { label: "Active", key: "Active" },
        { label: "Latest", key: "Latest" },
      ];
  }
}
