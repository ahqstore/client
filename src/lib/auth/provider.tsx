import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
} from "react";
import { genAuth, User } from ".";
import { tryAutoLogin } from "./login";

export const authObject = genAuth();

const Auth = createContext(authObject.currentUser);
export const useAuth = () => useContext(Auth);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [user, setUser] = useState<User | undefined>();

  useEffect(() => {
    authObject.onAuthChange.push((user) => {
      console.debug(user);
      setUser(user);
    });
    tryAutoLogin(authObject);
  }, []);

  return <Auth.Provider value={user}>{children}</Auth.Provider>;
};
