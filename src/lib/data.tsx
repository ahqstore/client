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
    console.log("Loading home...");

    setTimeout(() => {
      (async () => {
        const json = await getHome();

        setHome(json.home);
        setSplash(json.splash);

        console.log("Done");
      })()
    }, 1000);
  }, []);

  return (
    <Home.Provider value={home}>
      <Splash.Provider value={splash}>{children}</Splash.Provider>
    </Home.Provider>
  );
}
