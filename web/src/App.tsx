import { useState } from "react";
import AddressInput from "./components/AddressInput";
import BalanceDisplay from "./components/balance/Balance";
import { ModeToggle } from "./components/theme/ModeToggle";
import { ThemeProvider } from "./components/theme/ThemeProvider";

function App() {
	const [address, setAddress] = useState<string | null>(null);

	const apiBaseUrl = import.meta.env.VITE_API_BASE_URL;
	if (!apiBaseUrl) {
		throw new Error("VITE_API_BASE_URL is not set in the environment.");
	}

	return (
		<ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
			<div className="min-h-screen px-4 py-8 relative">
				<div className="absolute top-4 right-4">
					<ModeToggle />
				</div>

				<h1 className="text-4xl font-bold mb-8 text-center">Scanza</h1>

				<div className="mx-auto w-full max-w-md mb-6">
					<AddressInput onSubmit={(addr) => setAddress(addr)} />
				</div>

				{address && (
					<div className="mx-auto w-full max-w-2xl">
						<BalanceDisplay address={address} baseUrl={apiBaseUrl} />
					</div>
				)}
			</div>
		</ThemeProvider>
	);
}

export default App;
