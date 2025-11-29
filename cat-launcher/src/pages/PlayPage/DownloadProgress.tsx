import { Progress } from "@/components/ui/progress";

interface DownloadProgressProps {
  downloaded: number;
  total: number;
}

export function DownloadProgress({ downloaded, total }: DownloadProgressProps) {
  const progress = total > 0 ? (downloaded * 100) / total : 0;

  return (
    <Progress className="h-9 rounded-md" value={progress}>
      Downloading...
    </Progress>
  );
}
