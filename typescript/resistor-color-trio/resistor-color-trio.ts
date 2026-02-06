export function decodedResistorValue([
  color1,
  color2,
  color3,
]: string[]): string {
  const COLORS = [
    "black",
    "brown",
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "violet",
    "grey",
    "white",
  ];

  const value1 = COLORS.indexOf(color1);
  const value2 = COLORS.indexOf(color2);
  const zeros = COLORS.indexOf(color3);

  const ohms = (value1 * 10 + value2) * 10 ** zeros;

  if (ohms >= 1000000000) {
    return `${ohms / 1000000000} gigaohms`;
  }
  if (ohms >= 1000000) {
    return `${ohms / 1000000} megaohms`;
  }
  if (ohms >= 1000) {
    return `${ohms / 1000} kiloohms`;
  }
  return `${ohms} ohms`;
}
