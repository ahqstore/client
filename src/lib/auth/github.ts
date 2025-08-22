import { fetch } from "@tauri-apps/plugin-http";
import { Auth } from ".";
import { login } from "./login";

import { open, showCode, removeCode } from "tauri-plugin-ahqstore-api";

import { toast } from "sonner";

import { clientId, scopes } from "./server";

interface DeviceCode {
  device_code: string;
  user_code: string;
  verification_uri: string;
  expires_in: number;
  interval: number;
}

interface Value {
  access_token?: string;
}

export async function startLogin(auth: Auth) {
  const val: DeviceCode = await fetch(
    `https://github.com/login/device/code?client_id=${clientId}&scope=${scopes}`,
    {
      headers: {
        Accept: "application/json",
      },
      method: "POST",
    },
  ).then((r) => r.json());

  toast.success(`Opened ${val.verification_uri}`);

  toast(`Enter code: ${val.user_code}`, {
    cancel: {
      label: "Ok",
      onClick: () => {},
    },
    duration: 30_000,
  });

  open(val.verification_uri);

  showCode(val.user_code);

  let not_done = 0;

  const time = setInterval(async () => {
    const response: Value = await fetch(
      `https://github.com/login/oauth/access_token?client_id=${clientId}&device_code=${val.device_code}&grant_type=urn:ietf:params:oauth:grant-type:device_code`,
      {
        headers: {
          Accept: "application/json",
        },
        method: "POST",
      },
    ).then((r) => r.json());

    not_done += 1;

    if (response?.access_token != undefined) {
      removeCode();
      clearInterval(time);
      if (await login(auth, response.access_token)) {
        toast.success("Logged in");
      } else {
        toast.error("Failed to Log in");
      }
    }

    if (not_done >= 10) {
      removeCode();

      toast.error("Failed to login: Timed Out");
      clearInterval(time);
    }
  }, 6000);
}
