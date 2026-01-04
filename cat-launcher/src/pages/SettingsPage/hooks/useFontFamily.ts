import { useEffect, useRef, useState } from "react";

import { Font } from "@/generated-types/Font";
import { ensureFontLoaded } from "@/lib/utils";

export function useFontFamily(
  font: Font | null | undefined,
  onFontLoadError?: (error: unknown) => void,
) {
  const [fontFamily, setFontFamily] = useState<string | undefined>(
    undefined,
  );
  const onFontLoadErrorRef = useRef(onFontLoadError);

  useEffect(() => {
    onFontLoadErrorRef.current = onFontLoadError;
  }, [onFontLoadError]);

  useEffect(() => {
    if (!font) {
      setFontFamily(undefined);
      return;
    }

    let active = true;

    ensureFontLoaded(font.path)
      .then((family) => {
        if (active) {
          setFontFamily(family);
        }
      })
      .catch((error) => {
        if (active && onFontLoadErrorRef.current) {
          onFontLoadErrorRef.current(error);
        }
      });

    return () => {
      active = false;
    };
  }, [font]);

  return fontFamily;
}
