export const COLORS = [
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
] as const;

export const colorCode = (color: string): number =>
	COLORS.indexOf(color as (typeof COLORS)[number]);
