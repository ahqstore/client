import type { OsType } from "@tauri-apps/plugin-os";
import { useEffect, useState } from "react";
import { cn } from "@controls/libs/utils";
import { getOsType } from "./libs/plugin-os";
import type { WindowTitlebarProps } from "./types";
import { WindowControls } from "./window-controls";

export function WindowTitlebar({
  children,
  controlsOrder = "system",
  className,
  right,
  windowControlsProps,
  ...props
}: WindowTitlebarProps) {
  const [osType, setOsType] = useState<OsType | undefined>(undefined);

  useEffect(() => {
    getOsType().then((type) => {
      setOsType(type);
    });
  }, []);

  const left =
    controlsOrder === "left" ||
    (controlsOrder === "platform" &&
      windowControlsProps?.platform === "macos") ||
    (controlsOrder === "system" && osType === "macos");

  const customProps = (ml: string) => {
    if (windowControlsProps?.justify !== undefined) return windowControlsProps;

    const {
      justify: windowControlsJustify,
      className: windowControlsClassName,
      ...restProps
    } = windowControlsProps || {};
    return {
      justify: false,
      className: cn(windowControlsClassName, ml),
      ...restProps,
    };
  };

  return (
    <div
      className={cn(
        "bg-transparent flex select-none flex-row overflow-hidden",
        className,
      )}
      data-tauri-drag-region
      {...props}
    >
      {left ? (
        <>
          <WindowControls {...customProps("ml-0")} />
          {children}
          {right}
        </>
      ) : (
        <>
          {children}
          <WindowControls right={right} {...customProps("ml-auto")} />
        </>
      )}
    </div>
  );
}
