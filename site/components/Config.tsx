import { CONFIG_TOML } from "@/lib/product";

export default function Config() {
  return (
    <section id="config" className="border-t border-white/[0.04] px-5 py-14">
      <div className="mx-auto max-w-3xl">
        <p className="mb-4 text-[0.72rem] uppercase tracking-[0.18em] text-[var(--dim)]">Config · optional</p>
        <h2 className="mb-6 text-2xl font-bold tracking-tight">Sensible defaults. Everything overridable.</h2>
        <div className="overflow-x-auto rounded-xl border border-border bg-[var(--surface)] p-5">
          <pre className="text-[0.84rem] leading-7"><code>{CONFIG_TOML}</code></pre>
        </div>
        <p className="mt-6 text-[0.9rem] text-[var(--dim)]">
          <strong className="text-[var(--fg)]">How the cold-start cache works.</strong> Claude Code only knows your usage after its first API call — a fresh session would show blanks. ccstatus writes last-known usage to <code className="text-[var(--dim)]">~/.cache/ccstatus/usage.json</code> and reads it back on cold start. Live data always overrides and refreshes it. No network, no auth.
        </p>
      </div>
    </section>
  );
}
