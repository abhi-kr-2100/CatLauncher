import { usePlayTimeMonitor } from "@/hooks/usePlayTimeMonitor";
import { useAppSelector } from "@/store/hooks";

/**
 * A headless component that monitors the play time of the currently running game.
 * It uses the `usePlayTimeMonitor` hook to track and save the duration of game sessions.
 *
 * @returns null - This component does not render any visual elements.
 *
 * @public
 */
const PlayTimeMonitor = () => {
  const { currentlyPlaying, currentlyPlayingVersion } =
    useAppSelector((state) => state.gameSession);

  usePlayTimeMonitor(currentlyPlaying, currentlyPlayingVersion);

  return null;
};

export default PlayTimeMonitor;
