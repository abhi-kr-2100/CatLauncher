import type { DownloadProgress } from "@/generated-types/DownloadProgress";

/**
 * A serializable version of the {@link DownloadProgress} interface.
 * Useful for passing progress data between different parts of the application where BigInt might not be supported.
 */
export interface SerializableDownloadProgress {
  /**
   * The number of bytes downloaded so far.
   */
  bytes_downloaded: number;
  /**
   * The total number of bytes to download.
   */
  total_bytes: number;
}

/**
 * Converts a {@link DownloadProgress} object to a {@link SerializableDownloadProgress} object.
 *
 * @param progress - The download progress object to convert.
 * @returns A serializable version of the download progress.
 */
export function toSerializableDownloadProgress(
  progress: DownloadProgress,
): SerializableDownloadProgress {
  return {
    bytes_downloaded: Number(progress.bytes_downloaded),
    total_bytes: Number(progress.total_bytes),
  };
}
