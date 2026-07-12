import { ImageResponse } from "next/og";
import { VERSION } from "@/lib/product";

export const size = { width: 1200, height: 630 };
export const contentType = "image/png";
export const dynamic = "force-static";

export default function OgImage() {
  return new ImageResponse(
    (
      <div style={{ width: "100%", height: "100%", background: "#0a0b0e", color: "#eceae4", display: "flex", flexDirection: "column", justifyContent: "center", padding: 80, fontFamily: "monospace" }}>
        <div style={{ color: "#7c8088", fontSize: 26, marginBottom: 18 }}>{`Claude Code status line · v${VERSION}`}</div>
        <div style={{ display: "flex", fontSize: 64, fontWeight: 700, lineHeight: 1.1 }}>
          Your usage,&nbsp;<span style={{ color: "#d7875f" }}>already on screen</span>
        </div>
        <div style={{ marginTop: 28, fontSize: 30, color: "#d7875f" }}>████▓▓▁▁▁▁▁▁▁▁▁▁▁▁  22%</div>
        <div style={{ marginTop: 40, fontSize: 30, color: "#7c8088" }}>ccstatus - one Rust binary . brew . npm . cargo</div>
      </div>
    ),
    { ...size }
  );
}
