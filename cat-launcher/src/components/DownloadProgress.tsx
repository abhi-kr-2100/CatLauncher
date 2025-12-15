import { Progress } from "@/components/ui/progress";

interface DownloadProgressProps {
  downloaded: bigint;
  total: bigint;
}

export function DownloadProgress({
  downloaded,
  total,
}: DownloadProgressProps) {
  const downloadedNumber = Number(downloaded);
  const totalNumber = Number(total);

  // For some downloads, the total size is not known.
  const showIndeterminateProgress =
    totalNumber === 0 && downloadedNumber > 0;

  const progress =
    totalNumber > 0 ? (downloadedNumber * 100) / totalNumber : 0;

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
