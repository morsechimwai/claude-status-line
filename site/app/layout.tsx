import "./globals.css";
import localFont from "next/font/local";
import type { Metadata } from "next";
import { VERSION, REPO, FAQ } from "@/lib/product";

const mono = localFont({
  src: [
    { path: "./fonts/JetBrainsMono-Regular.woff2", weight: "400", style: "normal" },
    { path: "./fonts/JetBrainsMono-Medium.woff2", weight: "500", style: "normal" },
    { path: "./fonts/JetBrainsMono-Bold.woff2", weight: "700", style: "normal" },
  ],
  variable: "--font-mono",
  display: "swap",
});

const SITE = process.env.NEXT_PUBLIC_SITE_URL ?? "https://ccstatus.vercel.app";

export const metadata: Metadata = {
  metadataBase: new URL(SITE),
  title: "ccstatus — a fast status line for Claude Code",
  description:
    "ccstatus renders your Claude Code model, context window, and 5-hour and 7-day rate-limit windows as hi-res braille bars — with cached usage shown instantly on session start. One Rust binary, no deps.",
  keywords: ["ccstatus", "Claude Code", "status line", "statusline", "braille", "usage", "rate limit", "Rust CLI"],
  authors: [{ name: "morsechimwai", url: REPO }],
  alternates: { canonical: "/" },
  openGraph: {
    type: "website", url: SITE, siteName: "ccstatus",
    title: "ccstatus — a fast status line for Claude Code",
    description: "Model, context, and rate-limit windows as hi-res braille bars. Cached usage on cold start. One Rust binary.",
  },
  twitter: { card: "summary_large_image", title: "ccstatus", description: "A fast status line for Claude Code." },
};

const jsonLd = {
  "@context": "https://schema.org",
  "@graph": [
    {
      "@type": "SoftwareApplication",
      name: "ccstatus",
      applicationCategory: "DeveloperApplication",
      operatingSystem: "macOS, Linux, Windows",
      softwareVersion: VERSION,
      license: "https://opensource.org/licenses/MIT",
      url: SITE,
      downloadUrl: `${REPO}/releases`,
      offers: { "@type": "Offer", price: "0", priceCurrency: "USD" },
      description: "A fast status line for Claude Code that renders usage as hi-res braille bars.",
    },
    {
      "@type": "FAQPage",
      mainEntity: FAQ.map((f) => ({
        "@type": "Question", name: f.q,
        acceptedAnswer: { "@type": "Answer", text: f.a },
      })),
    },
  ],
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={mono.variable}>
      <body>
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd).replace(/</g, "\\u003c") }}
        />
        {children}
      </body>
    </html>
  );
}
