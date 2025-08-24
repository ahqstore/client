import React from "react";

interface ConfigProps {
  title: string;
  description: string;
  Icon: typeof ChevronDown;
  children: React.ReactNode;
  pointer?: boolean;
}

import { ChevronDown } from "lucide-react";

export function ConfigSelect({
  title,
  description,
  Icon,
  children,
  pointer
}: ConfigProps) {
  return (
    <div
      className={`bg-accent dark:bg-neutral-content/10 animate w-full rounded-lg`}
    >
      <div
        className={`w-full flex ${pointer ? "cursor-pointer" : "cursor-default"} text-muted-content dark:text-foreground category p-3`}
      >
        <div className="size-10 my-auto">
          <Icon size="2.25rem" className="m-auto" />
        </div>
        <div className="ml-2 w-full">
          <h1 className="text-lg select-none">{title}</h1>
          <span className="select-none">{description}</span>
        </div>
        <div className="p-1 my-auto rounded-md">
          {children}
        </div>
      </div>
    </div>
  );
}
