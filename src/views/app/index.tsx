import { openApplicationState } from "@/data/implementations/appView";
import { useStore } from "@/data/store";

export default function Application() {
  const appId = useStore(openApplicationState);

  console.log(appId);

  return <></>;
}