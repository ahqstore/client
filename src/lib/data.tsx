import { createContext, ReactNode, useContext, useEffect, useState } from "react";
import { getHome } from "tauri-plugin-ahqstore-api";

const Home = createContext<[string, string[]][] | undefined>(undefined);

export const useHome = () => useContext(Home);

export function HomeProvider({ children }: { children: ReactNode }) {
  const [home, setHome] = useState<[string, string[]][] | undefined>(undefined);

  useEffect(() => {
    getHome().then(setHome);
  }, []);

  return (
    <Home.Provider value={home}>
      {children}
    </Home.Provider>
  );
}