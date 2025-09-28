import { useQuery } from '@tanstack/react-query';

import type { GameVariantInfo } from "./generated-types/GameVariantInfo";
import type { GameRelease } from './generated-types/GameRelease';
import { fetchReleasesForVariant } from "@/lib/utils";

export interface GameVariantProps {
    variant: GameVariantInfo;
}

export default function GameVariant(props: GameVariantProps) {
  const { variant } = props;
  const { data: releases, isLoading, error } = useQuery<GameRelease[]>({
    queryKey: ["releases", variant.name],
    queryFn: () => fetchReleasesForVariant(variant),
  });
  
  return (
    <div>
      <h2>{variant.name}</h2>
      <p>{variant.description}</p>
      <h3>Releases</h3>
      {isLoading && <p>Loading releases...</p>}
      {error && <p>Error loading releases: {String(error)}</p>}
      {releases && releases.length > 0 ? (
        <ul>
          {releases.map((release) => (
            <li key={release.variant}>
              {release.version}
            </li>
          ))}
        </ul>
      ) : !isLoading && <p>No releases found.</p>}
    </div>
  );
}
