export default function Background() {
  return (
    <div
      className="absolute inset-0 -z-10 flex justify-center"
      style={{ top: '-282px' }}
    >
      <div className="bg-shape bg-mainGradientStart opacity-50 bg-blur translate-x-20"></div>
      <div className="bg-shape bg-mainGradientEnd opacity-50 bg-blur -translate-x-20"></div>
    </div>
  );
}
