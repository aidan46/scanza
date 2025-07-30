export function normalizeString(value: string): string {
	if (!value) return "";
	return value.replace(/^"+|"+$/g, "").toLowerCase();
}

export function shortHash(hash: string): string {
	const clean = normalizeString(hash);
	return `${clean.slice(0, 8)}...${clean.slice(-6)}`;
}

export function formatFunctionName(signature: string): string {
	const match = signature.match(/^([a-zA-Z0-9_]+)\s*\(/);
	if (!match) return "Contract Call";
	const raw = match[1];
	return raw.charAt(0).toUpperCase() + raw.slice(1);
}

export function timeAgo(timestamp: string): string {
	const now = Date.now();
	const seconds = now / 1000 - parseInt(timestamp, 10);
	const minutes = Math.floor(seconds / 60);
	const hours = Math.floor(minutes / 60);
	if (minutes < 1) return "just now";
	if (minutes < 60) return `${minutes} min${minutes > 1 ? "s" : ""} ago`;
	if (hours < 24) return `${hours} hour${hours > 1 ? "s" : ""} ago`;
	return new Date(parseInt(timestamp, 10) * 1000).toLocaleString();
}
