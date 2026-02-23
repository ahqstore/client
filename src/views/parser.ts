import { toast } from "sonner";
import { DeepLinkMeta } from "./deeplink";

import { getHomeRef } from "@/lib/data";

const regexp = /^ahqstore:\/\/(?:(?<type>app|category)\/(?<id>[a-zA-Z0-9:.]+))$/;

let lastToastTime = 0;
const TOAST_COOLDOWN = 2000; // 2 seconds

function showSecureToast(message: string, description: string) {
  const now = Date.now();

  if (now - lastToastTime > TOAST_COOLDOWN) {
    toast.error(message, { description });
    lastToastTime = now;
  }
}

export function matchParseDeepLink(data: string): DeepLinkMeta | null {
  const out = data.match(regexp);

  if (!out || !out.groups) {
    showSecureToast("Invalid Deep Link", "A deep link from an application was created with completely invalid and garbage data. ERR_INVALID_DEEPLINK_SCHEMA");

    return null;
  }

  const { id, type } = out.groups;

  if (type == "app") {
    return {
      isOpen: true,
      task: {
        type: "app",
        id
      }
    };
  } else {
    const num = parseInt(id, 10);

    const home = getHomeRef();

    if (!isNaN(num) && num.toString() === id && home && num >= 0 && num < home.length) {
      return {
        isOpen: true,
        task: {
          type: "category",
          id: num
        }
      }
    } else {
      showSecureToast("Invalid Deep Link", "A deep link from an application was created with invalid navigation data. ERR_INVALID_GROUP_ID");

      return null;
    }
  }
}