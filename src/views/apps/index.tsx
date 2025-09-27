import { useHome, useSplash } from "@/lib/data";
import { ArrowRight, Search } from "lucide-react";

import { ChevronRight } from "lucide-react";
import { AppBox } from "./appBox";
import { categoryView } from "@/data/implementations/catView";
import { openApplicationState } from "@/data/implementations/appView";
import { useMediaQuery } from "@/hooks/use-media-query";
import { useRef } from "react";
import { searchQueryData } from "@/data/implementations/searchData";

export function AppsHome({ set }: { set: (_: number) => void }) {
  const splash = useSplash();
  const home = useHome();

  const isTablet = useMediaQuery("(min-width: 50rem)");

  const inputRef = useRef<HTMLInputElement>(null);

  return (
    <>
      <div className="w-full h-full overflow-hidden flex flex-col">
        <div className="flex w-full justify-center text-center items-center">
          <div className="relative w-full max-w-xl">
            <form
              onSubmit={(e) => {
                e.preventDefault();
                const val = inputRef.current!!.value;

                inputRef.current!!.value = "";

                searchQueryData.data = val;
                set(9);
              }}
            >
              <input
                type="text"
                placeholder="Search for apps, games, and more"
                className="w-full py-3 pl-9 pr-3 bg-primary/10 rounded-xl border border-border focus:outline-none focus:ring-1 focus:ring-muted transition duration-300"
                ref={inputRef}
                minLength={3}
              />
              <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground h-5 w-5" />
            </form>
          </div>
        </div>

        <div className="flex flex-col mt-2 w-full h-full overflow-y-scroll overflow-x-hidden">
          {splash && (
            <>
              <div className="flex flex-col mt-3 lg:flex-row grow-0 w-full gap-5 animate">
                <div
                  className="hero"
                  style={{
                    background: `url("${splash.hero.background}") center/cover`,
                  }}
                  onClick={() => {
                    if (!isTablet) {
                      openApplicationState.data = splash.hero.appId;
                      set(10);
                    }
                  }}
                >
                  <div></div>
                  <div className="flex flex-col h-full">
                    <h1>{splash.hero.title}</h1>
                    <h2 className="">{splash.hero.description}</h2>

                    <div className="w-full">
                      <button
                        onClick={() => {
                          openApplicationState.data = splash.hero.appId;
                          set(10);
                        }}
                      >{splash.hero.button}</button>
                    </div>

                    <h3 className="mt-auto mb-2 hidden md:block">
                      {splash.hero.author}
                    </h3>
                  </div>
                </div>

                <div className="cards">
                  <div
                    onClick={() => {
                      openApplicationState.data = splash.subhero.appId;
                      set(10);
                    }}
                  >
                    <div style={{ color: splash.subhero.color }}>
                      {splash.subhero.title}
                    </div>
                    <img src={splash.subhero.background} />
                  </div>

                  <div
                    onClick={() => {
                      openApplicationState.data = splash.third.appId;
                      set(10);
                    }}
                  >
                    <div style={{ color: splash.third.color }}>
                      {splash.third.title}
                    </div>
                    <img src={splash.third.background} />
                  </div>

                  <div
                    onClick={() => {
                      openApplicationState.data = splash.fourth.appId;
                      set(10);
                    }}
                  >
                    <div style={{ color: splash.fourth.color }}>
                      {splash.fourth.title}
                    </div>
                    <img src={splash.fourth.background} />
                  </div>
                </div>
              </div>
            </>
          )}

          {home?.map((apps, index) => (
            <div
              className="home_apps"
              key={apps[0]}
            >
              <div
                onClick={() => {
                  if (isTablet) {
                    categoryView.data = index;
                    set(11);
                  }
                }}
              >
                <span>{apps[0]}</span>
                <ChevronRight
                  className="arrow"
                  color="color-mix(in srgb, var(--color-neutral-content) 5%, var(--win32-accent) 75%)"
                />
                <button
                  className="all"
                  onClick={() => {
                    categoryView.data = index;
                    set(11);
                  }}
                >
                  <ArrowRight className="arrow" />
                </button>
              </div>
              <div>
                {apps[1].slice(0, 4).map((app) => (
                  <AppBox key={`${app}${apps[0]}`} appId={app} set={set} />
                ))}
              </div>
            </div>
          ))}
        </div>
      </div>
    </>
  );
}
