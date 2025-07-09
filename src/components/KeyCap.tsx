export const KeyCap = ({ color }: { color: string }) => (
  <div
    className="key"
    style={{ "--kc-base": color } as React.CSSProperties}
  >
    <div className="keycap" />
  </div>
);
