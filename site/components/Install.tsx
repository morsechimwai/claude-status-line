"use client";
import { useState } from "react";
import { INSTALL, SETTINGS_SNIPPET } from "@/lib/product";

export default function Install() {
  const [tab, setTab] = useState<(typeof INSTALL)[number]["id"]>(INSTALL[0].id);
  const [copied, setCopied] = useState(false);
  const active = INSTALL.find((i) => i.id === tab)!;

  const copy = async () => {
    try { await navigator.clipboard.writeText(active.command); setCopied(true); setTimeout(() => setCopied(false), 1400); } catch {}
  };

  return (
    <section id="install" className="border-t border-white/[0.04] px-5 py-14">
      <div className="mx-auto max-w-3xl">
        <p className="mb-4 text-[0.72rem] uppercase tracking-[0.18em] text-[var(--dim)]">Install</p>
        <h2 className="mb-6 text-2xl font-bold tracking-tight">Pick your package manager.</h2>
        <div role="tablist" aria-label="Install method" className="flex gap-1">
          {INSTALL.map((i) => (
            <button
              key={i.id} role="tab" aria-selected={i.id === tab}
              onClick={() => setTab(i.id)}
              className="cursor-pointer rounded-t-md border border-b-0 px-3.5 py-2 text-[0.82rem] transition-colors"
              style={{
                color: i.id === tab ? "var(--accent)" : "var(--dim)",
                background: i.id === tab ? "var(--surface)" : "transparent",
                borderColor: "var(--border)",
              }}
            >{i.label}</button>
          ))}
        </div>
        <div className="flex items-center justify-between gap-4 overflow-x-auto rounded-b-lg rounded-tr-lg border border-border bg-[var(--surface)] px-[1.15rem] py-4">
          <code className="whitespace-pre text-[0.9rem]">
            <span className="text-[var(--dim2)]">{active.comment}</span>{"\n"}{active.command}
          </code>
          <button onClick={copy} className="flex-none cursor-pointer rounded-md border border-border px-2.5 py-1.5 text-[0.72rem] text-[var(--dim)] transition-colors hover:text-[var(--fg)]">
            {copied ? "copied ✓" : "copy"}
          </button>
        </div>
        <p className="mt-4 text-[0.85rem] text-[var(--dim)]">
          Then point Claude Code at it in <code className="text-[var(--accent)]">~/.claude/settings.json</code>:<br />
          <code className="text-[var(--accent)]">{SETTINGS_SNIPPET}</code>
        </p>
      </div>
    </section>
  );
}
