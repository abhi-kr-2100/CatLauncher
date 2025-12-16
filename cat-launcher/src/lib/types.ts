import type { DownloadProgress } from "@/generated-types/DownloadProgress";

export interface SerializableDownloadProgress {
  bytes_downloaded: number;
  total_bytes: number;
}

export function toSerializableDownloadProgress(
  progress: DownloadProgress,
): SerializableDownloadProgress {
  return {
    bytes_downloaded: Number(progress.bytes_downloaded),
    total_bytes: Number(progress.total_bytes),
  };
}
