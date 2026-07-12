const PATHS = {
  sparkle: "M12 2v4M12 18v4M2 12h4M18 12h4",
  bars: "M4 18V9M9 18V5M14 18v-7M19 18v-4",
  clock: "M12 7v5l3 2",
  check: "M20 7L9 18l-5-5",
  shield: "M12 2l8 4v6c0 5-3.5 8-8 10-4.5-2-8-5-8-10V6z",
  sliders: "M4 6h16M4 12h10M4 18h7",
};

export default function Icon({ name }: { name: keyof typeof PATHS }) {
  return (
    <svg viewBox="0 0 24 24" width="26" height="26" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
      {name === "clock" && <circle cx="12" cy="12" r="9" />}
      {name === "sparkle" && <circle cx="12" cy="12" r="4" />}
      <path d={PATHS[name]} />
    </svg>
  );
}
