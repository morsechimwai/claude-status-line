#!/usr/bin/env python3
# Render a REAL screenshot from the actual ccstatus binary output.
# Reads ANSI bytes from stdin (or a file arg) and rasterises them with the
# same monospace font a terminal uses, inside macOS-style window chrome.
import re, sys

from PIL import Image, ImageDraw, ImageFont

SRC = open(sys.argv[1], "rb").read().decode() if len(sys.argv) > 1 else sys.stdin.read()
OUT = sys.argv[2] if len(sys.argv) > 2 else "assets/screenshot.png"

# xterm-256 -> hex for the codes ccstatus actually emits.
XTERM = {
    245: "#8a8a8a", 240: "#585858", 173: "#d7875f",   # mono / track / orange
    68: "#5f87d7", 17: "#26306a",                        # blue preset fill / track
    72: "#5faf87", 22: "#1f4a2f",                        # green preset
    140: "#af87d7", 54: "#3a2a5a",                        # purple preset
}
FG_DEFAULT = "#e6e4de"   # bold/plain text
BG = "#0a0b0e"
STROKE = "#20242c"
PROMPT_USER = "#6f9f6a"
PROMPT_DIM = "#565c66"
TITLE = "#7c8088"

S = 3                      # supersample for crisp retina-like output
FS = 15 * S
FONT = ImageFont.truetype("/System/Library/Fonts/Menlo.ttc", FS)
FONT_B = ImageFont.truetype("/System/Library/Fonts/Menlo.ttc", FS)  # Menlo bold via synth below

# cell metrics from the font
asc, desc = FONT.getmetrics()
CW = FONT.getlength("M")
LH = int((asc + desc) * 1.32)

PAD = 22 * S
TITLEBAR = 34 * S
GAP = 14 * S               # gap under prompt/header handled by line flow


def parse(line):
    """Yield (text, color, bold) runs from one ANSI line."""
    runs, color, bold = [], FG_DEFAULT, False
    for tok in re.split(r"(\x1b\[[0-9;]*m)", line):
        if not tok:
            continue
        if tok.startswith("\x1b["):
            codes = tok[2:-1].split(";")
            i = 0
            while i < len(codes):
                c = codes[i]
                if c in ("", "0"):
                    color, bold = FG_DEFAULT, False
                elif c == "1":
                    bold = True
                elif c == "38" and i + 2 < len(codes) and codes[i + 1] == "5":
                    color = XTERM.get(int(codes[i + 2]), FG_DEFAULT)
                    i += 2
                i += 1
        else:
            runs.append((tok, color, bold))
    return runs


lines = SRC.rstrip("\n").split("\n")
# We prepend a shell prompt line for context, like a real terminal.
prompt = [("morse", PROMPT_USER, True), (" ~/project $ claude", PROMPT_DIM, False)]
render_lines = [prompt] + [parse(l) for l in lines]

# canvas size
max_cols = max(sum(len(t) for t, _, _ in rl) for rl in render_lines)
W = int(PAD * 2 + max_cols * CW) + 6 * S
H = TITLEBAR + PAD + len(render_lines) * LH + PAD // 2

img = Image.new("RGB", (W, H), BG)
d = ImageDraw.Draw(img)
# rounded window
mask = Image.new("L", (W, H), 0)
ImageDraw.Draw(mask).rounded_rectangle([0, 0, W - 1, H - 1], radius=12 * S, fill=255)
bg = Image.new("RGB", (W, H), BG)
img = Image.composite(bg, Image.new("RGB", (W, H), (0, 0, 0)), mask)
d = ImageDraw.Draw(img)
d.rounded_rectangle([S, S, W - 1 - S, H - 1 - S], radius=12 * S, outline=STROKE, width=S)
# title bar divider + traffic lights
d.line([(0, TITLEBAR), (W, TITLEBAR)], fill="#14171d", width=S)
for i, c in enumerate(("#ec6a5e", "#f4bf4f", "#61c554")):
    cx = (20 + i * 20) * S
    d.ellipse([cx - 5 * S, TITLEBAR // 2 - 5 * S, cx + 5 * S, TITLEBAR // 2 + 5 * S], fill=c)
tf = ImageFont.truetype("/System/Library/Fonts/Menlo.ttc", 11 * S)
d.text(((20 + 3 * 20 + 10) * S, TITLEBAR // 2 - 7 * S), "morse — claude", font=tf, fill=TITLE)

# Unicode braille (U+2800+n) encodes an 8-dot 2x4 grid via a bitmask.
# Menlo has no braille glyph, so we draw the real dots ourselves — pixel-faithful
# to the exact bytes the binary emitted.
DOT_BIT = {  # bit -> (col, row) in a 2-wide, 4-tall cell
    0x01: (0, 0), 0x02: (0, 1), 0x04: (0, 2), 0x40: (0, 3),
    0x08: (1, 0), 0x10: (1, 1), 0x20: (1, 2), 0x80: (1, 3),
}


def draw_braille(x, y, ch, color):
    n = ord(ch) - 0x2800
    r = 1.15 * S
    # dot grid geometry inside one character cell
    colx = [x + CW * 0.34, x + CW * 0.66]
    top = y + asc * 0.22
    rowy = [top + j * (asc * 0.62 / 3.0) for j in range(4)]
    for bit, (c, rw) in DOT_BIT.items():
        if n & bit:
            cx, cy = colx[c], rowy[rw]
            d.ellipse([cx - r, cy - r, cx + r, cy + r], fill=color)


y = TITLEBAR + PAD
for rl in render_lines:
    x = PAD
    for text, color, bold in rl:
        for ch in text:
            if 0x2800 <= ord(ch) <= 0x28FF:
                draw_braille(x, y, ch, color)
            else:
                d.text((x, y), ch, font=FONT, fill=color)
                if bold:  # faux-bold: re-stamp with sub-pixel offset
                    d.text((x + max(1, S // 2), y), ch, font=FONT, fill=color)
            x += CW
    y += LH

img.save(OUT)
print(f"wrote {OUT} {W//S}x{H//S} (@{S}x = {W}x{H}px)")
