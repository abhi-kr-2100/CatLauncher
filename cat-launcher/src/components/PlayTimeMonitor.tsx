import { usePlayTimeMonitor } from "@/hooks/usePlayTimeMonitor";
import { useAppSelector } from "@/store/hooks";

const PlayTimeMonitor = () => {
  const { currentlyPlaying, currentlyPlayingVersion } =
    useAppSelector((state) => state.gameSession);

  usePlayTimeMonitor(currentlyPlaying, currentlyPlayingVersion);

  return null;
};

export default PlayTimeMonitor;
