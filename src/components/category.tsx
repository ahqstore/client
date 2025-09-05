import React, { useState } from "react";

import { Separator } from "@/components/ui/separator";

interface CategoryProps {
  title: string;
  description: string;
  Icon: typeof ChevronDown;
  children?: React.ReactNode;
  nearChevron?: React.ReactNode;
  openable?: boolean;
  normallyOpen?: boolean;
}

import { ChevronDown } from "lucide-react";

export function Category({
  title,
  description,
  Icon,
  children,
  nearChevron,
  openable = true,
  normallyOpen,
}: CategoryProps) {
  const [open, setOpen] = useState(normallyOpen || false);

  return (
    <div
      className={`bg-accent dark:bg-neutral-content/10 animate w-full rounded-lg`}
    >
      <div
        className={`category w-full flex cursor-pointer text-muted-content dark:text-foreground p-3`}
        data-open={open ? "true" : "false"}
        onClick={() => {
          if (openable) {
            setOpen((o) => !o)
          }
        }}
      >
        <div className="size-10 my-auto">
          <Icon size="2.25rem" className="m-auto" />
        </div>
        <div className="ml-2 w-full">
          <h1 className="text-lg select-none">{title}</h1>
          <span className="select-none !text-foreground/80">{description}</span>
        </div>
        <div className="p-1 my-auto rounded-md">
          {nearChevron}
        </div>
        <div className="chv p-1 my-auto rounded-md">
          {openable && <ChevronDown size="1.75em" style={open ? { rotate: "180deg" } : {}} />}
        </div>
      </div>

      {openable && <Separator hidden={!open} />}

      <div className="p-3" hidden={!open}>
        {children}
      </div>
    </div>
  );
}
