import { useState } from "react";
import AddressInput from "./components/AddressInput";
import BalanceDisplay from "./components/BalanceDisplay";

function App() {
	const [address, setAddress] = useState<string | null>(null);

	const apiBaseUrl = import.meta.env.VITE_API_BASE_URL;
	if (!apiBaseUrl) {
		throw new Error("VITE_API_BASE_URL is not set in the environment.");
	}
	console.log("API_BASE_URL:", apiBaseUrl);

	return (
		<div>
			<h1>Scanza</h1>
			<AddressInput onSubmit={(addr) => setAddress(addr)} />
			{address && (
				<>
					<p>Looking up address: {address}</p>
					<BalanceDisplay address={address} baseUrl={apiBaseUrl} />
				</>
			)}
		</div>
	);
}

export default App;
