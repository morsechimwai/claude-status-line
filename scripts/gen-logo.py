#!/usr/bin/env python3
# Generate the ccstatus hero banner: braille-gauge mark + wordmark + tagline,
# on a self-contained dark card so it reads on light *and* dark README themes.
FILL = "#d7875f"      # Claude-brand orange (xterm 173)
TRACK = "#4a4a4a"
TEXT = "#eceae4"
DIM = "#7c8088"
BG = "#0a0b0e"
STROKE = "#20242c"

W, H = 500, 150

# braille gauge mark (left)
CELLS = 6
FILLED = 4
r = 3.2
gx0 = 44
gy0 = 52
cell_w = 18.0
col_dx = 8.0
row_dy = 15.0

dots = []
for c in range(CELLS):
    lit = c < FILLED
    color = FILL if lit else TRACK
    cx0 = gx0 + c * cell_w
    for col in range(2):
        for row in range(4):
            if not lit and row != 3:   # track cells: baseline row only (⣀)
                continue
            cx = cx0 + col * col_dx
            cy = gy0 + row * row_dy
            dots.append(f'<circle cx="{cx:.1f}" cy="{cy:.1f}" r="{r}" fill="{color}"/>')

svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="ui-monospace,'SF Mono','JetBrains Mono',Menlo,Consolas,monospace">
  <title>ccstatus</title>
  <rect x="1" y="1" width="{W-2}" height="{H-2}" rx="22" fill="{BG}" stroke="{STROKE}" stroke-width="2"/>
  <g>{''.join(dots)}</g>
  <text x="184" y="93" font-size="56" font-weight="700" letter-spacing="-1.5"><tspan fill="{FILL}">cc</tspan><tspan fill="{TEXT}">status</tspan></text>
</svg>
'''
open("assets/logo.svg", "w").write(svg)
print("wrote assets/logo.svg")
