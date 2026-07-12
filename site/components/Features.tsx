import Icon from "./Icon";
import { FEATURES } from "@/lib/product";

export default function Features() {
  return (
    <section id="features" className="border-t border-white/[0.04] px-5 py-14">
      <div className="mx-auto max-w-3xl">
        <p className="mb-4 text-[0.72rem] uppercase tracking-[0.18em] text-[var(--dim)]">Features</p>
        <h2 className="mb-6 text-2xl font-bold tracking-tight">Built for the terminal, not around it.</h2>
        <div className="grid gap-4 sm:grid-cols-2">
          {FEATURES.map((f) => (
            <div key={f.title} className="rounded-xl border border-border bg-[var(--surface)] p-5">
              <div className="mb-2.5 text-[var(--accent)]"><Icon name={f.icon} /></div>
              <h3 className="mb-1.5 font-bold">{f.title}</h3>
              <p className="text-[0.86rem] leading-relaxed text-[var(--dim)]">{f.body}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
