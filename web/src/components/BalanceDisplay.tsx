import { useEffect, useState } from "react";

type BalanceDisplayProps = {
	address: string;
	baseUrl: string;
};

type BalanceResponse = {
	address: string;
	balance: string;
};

export default function BalanceDisplay({
	address,
	baseUrl,
}: BalanceDisplayProps) {
	const [balance, setBalance] = useState<string | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [loading, setLoading] = useState(true);

	useEffect(() => {
		const fetchBalance = async () => {
			try {
				const res = await fetch(`${baseUrl}/wallet/${address}/balance`);
				if (!res.ok) {
					const errorText = await res.text();
					throw new Error(`HTTP ${res.status}: ${errorText}`);
				}

				const data: BalanceResponse = await res.json();
				const raw = BigInt(data.balance); // works with hex
				const formatted = (raw / 10n ** 18n).toString();
				setBalance(formatted);
			} catch (err) {
				setError(`Failed to fetch balance: ${err}`);
			} finally {
				setLoading(false);
			}
		};

		fetchBalance();
	}, [address, baseUrl]);

	if (loading) return <p>Loading balance...</p>;
	if (error) return <p style={{ color: "red" }}>{error}</p>;

	return (
		<p>
			Balance: <strong>{balance} ETH</strong>
		</p>
	);
}
