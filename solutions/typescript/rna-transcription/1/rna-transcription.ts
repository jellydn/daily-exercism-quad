export function toRna(dna: string): string {
	const rnaMap: { [key: string]: string } = {
		C: "G",
		G: "C",
		A: "U",
		T: "A",
	};

	for (const nucleotide of dna) {
		if (!rnaMap[nucleotide]) {
			throw new Error("Invalid input DNA.");
		}
	}

	return dna
		.split("")
		.map((nucleotide) => rnaMap[nucleotide])
		.join("");
}
