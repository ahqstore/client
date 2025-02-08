import { useEffect, useState } from "react"
import { cn } from "@controls/libs/utils"
import { TauriAppWindowProvider } from "./contexts/plugin-window"
import { Gnome, MacOS, Windows } from "./controls"
import { getOsType } from "./libs/plugin-os"
import type { WindowControlsProps } from "./types"

export function WindowControls({
  platform,
  justify = false,
  hide = false,
  hideMethod = "display",
  right,
  // linuxDesktop = "gnome",
  className,
  ...props
}: WindowControlsProps) {
  const [osType, setOsType] = useState<string | undefined>(undefined)

  useEffect(() => {
    getOsType().then((type) => {
      setOsType(type)
    })
  }, [])

  const customClass = cn(
    "flex",
    className,
    hide && (hideMethod === "display" ? "hidden" : "invisible")
  )

  // Determine the default platform based on the operating system if not specified
  if (!platform) {
    switch (osType) {
      case "macos":
        platform = "macos"
        break
      case "linux":
        platform = "gnome"
        break
      case "windows":
        platform = "windows"
        break
      default:
        platform = undefined
    }
  }

  const ControlsComponent = () => {
    switch (platform) {
      case "windows":
        return (
          <Windows
            className={cn(customClass, justify && "ml-auto")}
            children={right}
            {...props}
          />
        )
      case "macos":
        return (
          <MacOS children={right} className={cn(customClass, justify && "ml-0")} {...props} />
        )
      case "gnome":
        return (
          <Gnome children={right} className={cn(customClass, justify && "ml-auto")} {...props} />
        )
      default:
        return (
          <></>
        )
    }
  }

  return (
    <TauriAppWindowProvider>
      <ControlsComponent />
    </TauriAppWindowProvider>
  )
}
