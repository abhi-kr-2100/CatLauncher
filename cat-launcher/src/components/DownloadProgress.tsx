import { Progress } from "@/components/ui/progress";

interface DownloadProgressProps {
  downloaded: number;
  total: number;
}

export function DownloadProgress({
  downloaded,
  total,
}: DownloadProgressProps) {
  // For some downloads, the total size is not known.
  const showIndeterminateProgress = total === 0 && downloaded > 0;

  const progress = total > 0 ? (downloaded * 100) / total : 0;

  return (
    <Progress
      className={"h-9 rounded-md"}
      value={showIndeterminateProgress ? 0 : progress}
    >
      {showIndeterminateProgress
        ? "Downloading... This will take a while."
        : "Downloading..."}
    </Progress>
  );
}
