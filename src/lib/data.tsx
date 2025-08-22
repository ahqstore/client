import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import { getHome, Home as HomeInterface } from "tauri-plugin-ahqstore-api";

const Home = createContext<[string, string[]][] | undefined>(undefined);
const Splash = createContext<HomeInterface["splash"]>(undefined);

export const useHome = () => useContext(Home);
export const useSplash = () => useContext(Splash);

export function HomeProvider({ children }: { children: ReactNode }) {
  const [home, setHome] = useState<[string, string[]][] | undefined>(undefined);
  const [splash, setSplash] = useState<HomeInterface["splash"]>(undefined);

  useEffect(() => {
    getHome().then((json) => {
      setHome(json.home);
      setSplash(json.splash);
    });
  }, []);

  return (
    <Home.Provider value={home}>
      <Splash.Provider value={splash}>{children}</Splash.Provider>
    </Home.Provider>
  );
}
