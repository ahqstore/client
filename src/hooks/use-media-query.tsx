import { useEffect, useState } from "react";

export function useMediaQuery(mediaQuery: string) {
  const [query, setQuery] = useState(
    window.matchMedia(mediaQuery).matches
  );

  useEffect(() => {
    window.addEventListener("resize", (_) => {
      setQuery(window.matchMedia(mediaQuery).matches);
    });
  }, []);

  return query;
}