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
import { openLink, toastCL } from "@/lib/utils";
import useReleaseNotes from "./hooks/useReleaseNotes";

interface ReleaseNotesButtonProps {
  release: GameRelease;
}

export default function ReleaseNotesButton({
  release,
}: ReleaseNotesButtonProps) {
  const [isOpen, setIsOpen] = useState(false);

  const { notes, isLoading } = useReleaseNotes(release, (error) => {
    const errorMessage = `Failed to fetch release notes for ${release.version}.`;
    toastCL("error", errorMessage, error);
  });

  const releaseNotes = notes ?? release.body;

  return (
    <Dialog open={isOpen} onOpenChange={setIsOpen}>
      <DialogTrigger asChild>
        <Button variant="outline">
          <FileText className="h-4 w-4" />
          What's new?
        </Button>
      </DialogTrigger>
      <DialogContent className="flex max-h-[80vh] max-w-3xl flex-col">
        <DialogHeader>
          <DialogTitle>Release Notes</DialogTitle>
          <DialogDescription className="sr-only">
            Release notes for version {release.version}
          </DialogDescription>
        </DialogHeader>
        <div className="prose prose-sm dark:prose-invert flex-1 overflow-y-auto max-w-none px-1">
          {isLoading ? (
            "Loading..."
          ) : releaseNotes ? (
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
              {releaseNotes}
            </ReactMarkdown>
          ) : (
            "No release notes available."
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
