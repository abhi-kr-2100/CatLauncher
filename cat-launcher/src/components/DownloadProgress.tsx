import { Progress } from "@/components/ui/progress";
import { formatBytes } from "@/lib/utils";

/**
 * Properties for the {@link DownloadProgress} component.
 *
 * @public
 */
interface DownloadProgressProps {
  /** The number of bytes already downloaded. */
  downloaded: number;
  /** The total number of bytes to download. If 0, the progress is considered indeterminate. */
  total: number;
}

/**
 * A component that displays the progress of a file download.
 * Handles both determinate and indeterminate (unknown total size) progress states.
 *
 * @param props - The properties for the download progress component.
 * @returns A React element representing the download progress bar.
 *
 * @public
 */
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
