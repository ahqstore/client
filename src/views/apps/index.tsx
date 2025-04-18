import { useSplash } from "@/lib/data";
import { Search } from "lucide-react";

export function AppsHome() {
  const splash = useSplash();

  return <>
    <div className="flex w-full justify-center text-center items-center">
      <div className="relative w-full max-w-xl">
        <input
          type="text"
          placeholder="Search for apps, games, and more"
          className="w-full py-3 pl-9 pr-3 bg-primary/10 rounded-xl border border-border focus:outline-none focus:ring-1 focus:ring-muted transition duration-300"
        />
        <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground h-5 w-5" />
      </div>
    </div>

    {splash && <>
      <div className="flex flex-col mt-3 lg:flex-row grow-0 w-full gap-5 animate">
        <div className="hero" style={{ background: `url("${splash.hero.background}") center/cover` }}>
          <div></div>
          <div className="flex flex-col h-full">
            <h1>{splash.hero.title}</h1>
            <h2 className="">{splash.hero.description}</h2>

            <div className="w-full">
              <button>{splash.hero.button}</button>
            </div>

            <h3 className="mt-auto mb-2 hidden md:block">{splash.hero.author}</h3>
          </div>
        </div>

        <div className="cards">
          <div>
            <div>
              {splash.subhero.title}
            </div>
            <img src={splash.subhero.background} />
          </div>

          <div>
            <div>
              {splash.third.title}
            </div>
            <img src={splash.third.background} />
          </div>

          <div>
            <div>
              {splash.fourth.title}
            </div>
            <img src={splash.fourth.background} />
          </div>
        </div>
      </div>
    </>}
  </>;
}