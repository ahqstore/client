import React, { useState } from "react";

import { Separator } from "@/components/ui/separator";

interface CategoryProps {
  title: string;
  description: string;
  Icon: typeof ChevronDown;
  children: React.ReactNode;
  normallyOpen?: boolean;
}

import { ChevronDown } from "lucide-react";

export function Category({ title, description, Icon, children, normallyOpen }: CategoryProps) {
  const [open, setOpen] = useState(normallyOpen || false);

  return <div className={`bg-accent dark:bg-neutral-content/10 animate w-full rounded-lg`}>
    <div
      className={`w-full flex cursor-pointer text-muted-content dark:text-foreground category p-3`}
      onClick={() => setOpen((o) => !o)}
    >
      <div className="size-10 my-auto">
        <Icon
          size="2.25rem"
          className="m-auto"
        />
      </div>
      <div className="ml-2 w-full">
        <h1 className="text-lg select-none">{title}</h1>
        <span className="select-none">{description}</span>
      </div>
      <div className="chv p-1 my-auto rounded-md">
        <ChevronDown
          size="1.75em"
          style={open ? { rotate: "180deg" } : {}}
        />
      </div>
    </div>

    <Separator
      hidden={!open}
    />

    <div className="p-3" hidden={!open}>
      {children}
    </div>
  </div>;
}