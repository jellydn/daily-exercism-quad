export function decodedValue(colors: string[]): number {
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

	const value1 = COLORS.indexOf(colors[0]);
	const value2 = COLORS.indexOf(colors[1]);
	return value1 * 10 + value2;
}
