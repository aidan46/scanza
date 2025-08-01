import { createContext, useContext, useEffect, useState } from "react";

interface ChainsContextType {
	chains: string[];
	loading: boolean;
	error: string | null;
}

const ChainsContext = createContext<ChainsContextType>({
	chains: [],
	loading: true,
	error: null,
});

export function ChainsProvider({
	baseUrl,
	children,
}: {
	baseUrl: string;
	children: React.ReactNode;
}) {
	const [chains, setChains] = useState<string[]>([]);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);

	useEffect(() => {
		const fetchChains = async () => {
			try {
				const res = await fetch(`${baseUrl}/chains`);
				if (!res.ok) throw new Error(`Failed to load chains: ${res.status}`);
				const chains: string[] = await res.json();
				setChains(chains);
			} catch (err) {
				setError(`${err}`);
			} finally {
				setLoading(false);
			}
		};

		fetchChains();
	}, [baseUrl]);

	return (
		<ChainsContext.Provider value={{ chains, loading, error }}>
			{children}
		</ChainsContext.Provider>
	);
}

export function useChains() {
	return useContext(ChainsContext);
}
