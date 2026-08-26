import { useRef, useCallback, useEffect } from "react";

export function useThrottle<Args extends unknown[]>(
  func: (...args: Args) => void,
  delay: number,
): (...args: Args) => void {
  const lastCall = useRef(0);
  const timeoutId = useRef<ReturnType<typeof setTimeout> | null>(
    null,
  );
  const funcRef = useRef(func);
  funcRef.current = func;

  const throttledFunc = useCallback(
    (...args: Args): void => {
      const now = Date.now();
      const timeSinceLastCall = now - lastCall.current;

      if (timeSinceLastCall >= delay) {
        lastCall.current = now;
        funcRef.current(...args);
      } else {
        if (timeoutId.current) {
          clearTimeout(timeoutId.current);
        }
        timeoutId.current = setTimeout(() => {
          lastCall.current = Date.now();
          funcRef.current(...args);
        }, delay - timeSinceLastCall);
      }
    },
    [delay],
  );

  useEffect(() => {
    return () => {
      if (timeoutId.current) {
        clearTimeout(timeoutId.current);
      }
    };
  }, []);

  return throttledFunc;
}
