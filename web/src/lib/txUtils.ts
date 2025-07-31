export function formatAmount(value: number): string {
	if (value === 0) return "0";
	return value.toFixed(6).replace(/\.?0+$/, "");
}

export function normalizeString(value: string): string {
	if (!value) return "";
	return value.replace(/^"+|"+$/g, "");
}

export function shortHash(hash: string): string {
	const clean = normalizeString(hash);
	return `${clean.slice(0, 8)}...${clean.slice(-6)}`;
}

export function formatFunctionName(signature: string): string {
	const match = signature.match(/^([a-zA-Z0-9_]+)\s*\(/);
	if (!match) return "Contract Call";

	const raw = match[1];

	// Split by camelCase and underscores
	const words = raw
		.replace(/([a-z0-9])([A-Z])/g, "$1 $2") // camelCase → space
		.replace(/_/g, " ") // snake_case → space
		.split(" ");

	// Capitalize each word
	return words
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
		.join(" ");
}

export function timeAgo(timestamp: string): string {
	const now = Date.now();
	const seconds = Math.floor(now / 1000 - parseInt(timestamp, 10));
	const minutes = Math.floor(seconds / 60);
	const hours = Math.floor(minutes / 60);
	const days = Math.floor(hours / 24);

	if (seconds < 60) return "just now";
	if (minutes < 60) return `${minutes} min${minutes !== 1 ? "s" : ""} ago`;
	if (hours < 24) return `${hours} hour${hours !== 1 ? "s" : ""} ago`;
	return `${days} day${days !== 1 ? "s" : ""} ago`;
}
