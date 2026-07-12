import { FAQ } from "@/lib/product";

export default function Faq() {
  return (
    <section id="faq" className="border-t border-white/[0.04] px-5 py-14">
      <div className="mx-auto max-w-3xl">
        <p className="mb-4 text-[0.72rem] uppercase tracking-[0.18em] text-[var(--dim)]">FAQ</p>
        <h2 className="mb-6 text-2xl font-bold tracking-tight">Questions, answered.</h2>
        <div className="flex flex-col gap-2">
          {FAQ.map((f) => (
            <details key={f.q} className="rounded-lg border border-border bg-[var(--surface)] px-4 py-3">
              <summary className="cursor-pointer font-bold">{f.q}</summary>
              <p className="mt-2 text-[0.88rem] leading-relaxed text-[var(--dim)]">{f.a}</p>
            </details>
          ))}
        </div>
      </div>
    </section>
  );
}
