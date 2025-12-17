import { Progress } from "@/components/ui/progress";
import { formatBytes } from "@/lib/utils";

interface DownloadProgressProps {
  downloaded: number;
  total: number;
}

export function DownloadProgress({
  downloaded,
  total,
}: DownloadProgressProps) {
  // For some downloads, the total size is not known.
  const isIndeterminate = total === 0 && downloaded > 0;

  const progress = total > 0 ? (downloaded * 100) / total : 0;

  return (
    <Progress
      className={
        isIndeterminate
          ? "h-9 rounded-md animate-pulse"
          : "h-9 rounded-md"
      }
      value={isIndeterminate ? 0 : progress}
    >
      {isIndeterminate
        ? `Downloading... ${formatBytes(downloaded).join(" ")}`
        : "Downloading..."}
    </Progress>
  );
}
