import type { MetadataRoute } from "next";
const SITE = process.env.NEXT_PUBLIC_SITE_URL ?? "https://claude-status-line-ten.vercel.app";
export const dynamic = "force-static";
export default function sitemap(): MetadataRoute.Sitemap {
  return [{ url: SITE, changeFrequency: "monthly", priority: 1 }];
}
