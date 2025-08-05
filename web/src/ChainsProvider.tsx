import { createContext, useContext, useEffect, useState } from "react";

interface NativeCurrency {
	/// Full name of the native currency (e.g., "Ether")
	name: string;
	/// Symbol of the native currency (e.g., "ETH")
	symbol: string;
	/// Number of decimals used by the currency (typically 18)
	decimals: number;
}

interface ChainMetaData {
	/// Full name of the chain (e.g., "Ethereum Mainnet")
	name: string;
	/// EVM chain ID (e.g., 1 for Ethereum)
	chainId: number;
	/// Short name identifier (e.g., "eth")
	shortName: string;
	/// Network ID (sometimes differs from chain ID)
	networkId: number;
	/// Native currency metadata (name, symbol, decimals)
	nativeCurrency: NativeCurrency;
}

interface ChainsContextType {
	chains: ChainMetaData[];
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
	const [chains, setChains] = useState<ChainMetaData[]>([]);
	const [loading, setLoading] = useState(true);
	const [error, setError] = useState<string | null>(null);

	useEffect(() => {
		const fetchChains = async () => {
			try {
				const res = await fetch(`${baseUrl}/chains`);
				if (!res.ok) throw new Error(`Failed to load chains: ${res.status}`);
				const chains: ChainMetaData[] = await res.json();
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
