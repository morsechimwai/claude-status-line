import Hero from "@/components/Hero";
import Install from "@/components/Install";
import Features from "@/components/Features";
import Config from "@/components/Config";
import Faq from "@/components/Faq";
import { REPO } from "@/lib/product";

export default function Home() {
  return (
    <main>
      <Hero />
      <Install />
      <Features />
      <Config />
      <Faq />
      <footer className="border-t border-white/[0.04] px-5 py-12 text-center text-[0.8rem] text-[var(--dim2)]">
        ccstatus · MIT · <a href={REPO} className="text-[var(--dim)] hover:text-[var(--fg)]">github.com/morsechimwai/claude-status-line</a>
        <br />brew · npm · cargo · a single Rust binary
      </footer>
    </main>
  );
}
