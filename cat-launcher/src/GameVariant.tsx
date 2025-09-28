import { useQuery } from "@tanstack/react-query";
import { useMemo, useState } from "react";
import {
  Card,
  CardContent,
  CardDescription, CardHeader,
  CardTitle
} from "@/components/ui/card";
import Combobox, { ComboboxItem } from "@/components/ui/combobox";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { fetchReleasesForVariant } from "@/lib/utils";

export interface GameVariantProps {
  variant: GameVariantInfo;
}

export default function GameVariant(props: GameVariantProps) {
  const { variant } = props;
  const {
    data: releases,
    isLoading,
    error,
  } = useQuery<GameRelease[]>({
    queryKey: ["releases", variant.name],
    queryFn: () => fetchReleasesForVariant(variant),
  });

  const [selectedReleaseId, setSelectedReleaseId] = useState<
    string | undefined
  >();

  const comboboxItems = useMemo<ComboboxItem[]>(
    () =>
      releases?.map((r) => ({
        value: `${r.variant}-${r.version}`,
        label: `${r.version}`,
      })) ?? [],
    [releases]
  );

  const placeholderText = isLoading
    ? "Loading..."
    : error
    ? "Error loading releases."
    : comboboxItems.length === 0
    ? "No releases available."
    : "Select a release";

  return (
  <Card>
      <CardHeader>
        <CardTitle>{variant.name}</CardTitle>
        <CardDescription>
          <p className="text-sm text-muted-foreground line-clamp-3">
            {variant.description}
          </p>
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Combobox
          label="Version"
          items={comboboxItems}
          value={selectedReleaseId}
          onChange={setSelectedReleaseId}
          autoselect
          placeholder={placeholderText}
          disabled={isLoading || !!error || comboboxItems.length === 0}
        />
      </CardContent>
    </Card>
  );
}
