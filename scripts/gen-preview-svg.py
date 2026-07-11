#!/usr/bin/env python3
# Generate a terminal-style SVG of the real ccstatus output.
# Braille bars are drawn as vector dots so they render everywhere, no font needed.
import html

# palette (xterm -> hex)
LABEL = "#eceae4"; DIM = "#8a8a8a"; FILL = "#d7875f"; TRACK = "#6b6b6b"
TERM_BG = "#0a0b0e"; BAR_TITLE = "#7c8088"; PROMPT = "#565c66"
U = "#6f9f6a"; P = "#5f8aa8"

CW = 10.0            # char width
FS = 17              # font size
LH = 27              # line height
PAD_X = 22
TITLEBAR = 34
TOP = TITLEBAR + 20  # first text baseline offset area

WIDTH = 720
# lines: prompt + header + 3 rows = 5 text lines
N_LINES = 5
HEIGHT = TITLEBAR + 16 + N_LINES * LH + 18

def esc(s): return html.escape(s, quote=True)

parts = []
parts.append(f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{HEIGHT}" viewBox="0 0 {WIDTH} {HEIGHT}" font-family="ui-monospace,\'SF Mono\',\'JetBrains Mono\',Menlo,Consolas,monospace">')
parts.append(f'<rect x="0.5" y="0.5" width="{WIDTH-1}" height="{HEIGHT-1}" rx="12" fill="{TERM_BG}" stroke="#20242c"/>')
# title bar
parts.append(f'<line x1="0" y1="{TITLEBAR}" x2="{WIDTH}" y2="{TITLEBAR}" stroke="#ffffff" stroke-opacity="0.06"/>')
for i,c in enumerate(("#ec6a5e","#f4bf4f","#61c554")):
    parts.append(f'<circle cx="{20+i*20}" cy="{TITLEBAR/2}" r="5.5" fill="{c}"/>')
parts.append(f'<text x="{20+3*20+8}" y="{TITLEBAR/2+4}" font-size="12" fill="{BAR_TITLE}">morse — claude</text>')

def text(x_col, baseline, s, color, bold=False):
    w = ' font-weight="700"' if bold else ''
    x = PAD_X + x_col*CW
    return f'<text x="{x:.1f}" y="{baseline:.1f}" font-size="{FS}"{w} fill="{color}" xml:space="preserve">{esc(s)}</text>'

def braille_cell_dots(cell_col, line_top, filled_subcols, cell_index):
    """Draw one braille cell's dots. Returns svg fragment."""
    left = 2*cell_index < filled_subcols
    right = 2*cell_index + 1 < filled_subcols
    color = FILL if (left or right) else TRACK
    x = PAD_X + cell_col*CW
    # dot column x within cell
    cx = [x + CW*0.30, x + CW*0.64]
    # bar vertical band inside the line
    H = 15.0
    top = line_top - LH*0.62
    ry = [top + j*(H/3.0) for j in range(4)]  # rows 0..3
    r = 1.7
    dots = []
    # baseline dots 7,8 always (row 3, both cols)
    present = [(0,3),(1,3)]
    if left:  present += [(0,0),(0,1),(0,2)]
    if right: present += [(1,0),(1,1),(1,2)]
    for (col,row) in present:
        dots.append(f'<circle cx="{cx[col]:.2f}" cy="{ry[row]:.2f}" r="{r}" fill="{color}"/>')
    return "".join(dots)

def braille_bar(bar_col, baseline, pct, width=12):
    subcols = 2*width
    filled = (max(0,min(100,pct))*subcols + 50)//100
    out = []
    for i in range(width):
        out.append(braille_cell_dots(bar_col+i, baseline, filled, i))
    return "".join(out)

def row(y_line, label, pct, value):
    b = TOP + y_line*LH
    frag = []
    frag.append(text(0, b, label.ljust(8), LABEL, bold=True))
    frag.append(braille_bar(9, b, pct))
    frag.append(text(23, b, f"{pct:>3}%", DIM))
    frag.append(text(29, b, value, DIM))
    return "".join(frag)

# prompt line
b0 = TOP
parts.append(f'<text x="{PAD_X}" y="{b0:.1f}" font-size="{FS}" xml:space="preserve">'
             f'<tspan fill="{U}">morse</tspan><tspan fill="{PROMPT}"> ~/project $ claude</tspan></text>')
# header line
b1 = TOP + 1*LH
parts.append(text(0, b1, "Opus 4.8 (1M context)", LABEL, bold=True))
parts.append(text(23, b1, "Max (20x)", DIM))
# rows
parts.append(row(2, "Context", 22, "↑180k ↓45k / 1.0m"))
parts.append(row(3, "5h", 32, "resets in 2h 3m"))
parts.append(row(4, "7d", 33, "resets in 2d 7h"))

parts.append('</svg>')
open("assets/preview.svg","w").write("\n".join(parts))
print("wrote assets/preview.svg", HEIGHT, "px tall")
