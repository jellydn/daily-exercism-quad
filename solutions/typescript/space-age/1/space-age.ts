const EARTH_SECONDS_IN_YEAR = 31557600;

const PLANET_ORBITAL_PERIODS: Record<string, number> = {
	mercury: 0.2408467,
	venus: 0.61519726,
	earth: 1.0,
	mars: 1.8808158,
	jupiter: 11.862615,
	saturn: 29.447498,
	uranus: 84.016846,
	neptune: 164.79132,
};

export function age(planet: string, seconds: number): number {
	const orbitalPeriod = PLANET_ORBITAL_PERIODS[planet.toLowerCase()];

	if (orbitalPeriod === undefined) {
		throw new Error(`Invalid planet: ${planet}`);
	}

	const earthYears = seconds / EARTH_SECONDS_IN_YEAR;
	const planetYears = earthYears / orbitalPeriod;

	return Math.round(planetYears * 100) / 100;
}
