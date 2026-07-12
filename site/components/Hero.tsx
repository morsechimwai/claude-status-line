import TerminalDemo from "./TerminalDemo";
import { VERSION, REPO } from "@/lib/product";

export default function Hero() {
  return (
    <header className="px-5 pb-8 pt-14 text-center">
      <div className="mx-auto max-w-3xl">
        <p className="mb-4 text-[0.72rem] uppercase tracking-[0.18em] text-[var(--dim)]">
          Claude Code status line · single Rust binary · v{VERSION}
        </p>
        <h1 className="mx-auto mb-4 text-balance text-4xl font-bold leading-[1.08] tracking-tight sm:text-5xl">
          Your usage, <span className="glow text-[var(--accent)]">already on screen</span> before the first call.
        </h1>
        <p className="mx-auto mb-8 max-w-[42ch] text-balance text-[var(--dim)]">
          Model, context window, and both rate-limit windows as hi-res braille bars — with last-known usage cached and shown instantly on session start, not blanks.
        </p>
        <TerminalDemo />
        {/* visually-hidden text alternative for the demo (a11y + crawlers) */}
        <p className="sr-only">
          Terminal preview: model Opus 4.8 (1M context), plan label Max (20x). Context 22%, 5-hour window 9%, 7-day window 31%, each shown as a braille bar.
        </p>
        <div className="mt-8 flex flex-wrap justify-center gap-3">
          <a href="#install" className="glow inline-flex items-center rounded-lg bg-[var(--accent)] px-5 py-2.5 font-bold text-[#160d07] transition hover:brightness-110">$ install ccstatus</a>
          <a href={REPO} className="inline-flex items-center rounded-lg border border-border px-5 py-2.5 text-[var(--fg)] transition hover:border-[var(--dim2)]">View on GitHub →</a>
        </div>
        <p className="mt-1.5 text-[0.8rem] text-[var(--dim2)]">MIT · macOS / Linux / Windows · no runtime deps</p>
      </div>
    </header>
  );
}
