import { useEffect } from "react";

import { onFrontendReady } from "@/lib/commands";

export function useFrontendReady() {
  useEffect(() => {
    onFrontendReady();
  }, []);
}
