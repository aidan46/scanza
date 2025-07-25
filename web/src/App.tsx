import { useState } from "react";
import AddressInput from "./components/AddressInput";
import BalanceDisplay from "./components/BalanceDisplay";
import { ModeToggle } from "./components/ModeToggle";
import { ThemeProvider } from "./components/ThemeProvider";

function App() {
  const [address, setAddress] = useState<string | null>(null);

  const apiBaseUrl = import.meta.env.VITE_API_BASE_URL;
  if (!apiBaseUrl) {
    throw new Error("VITE_API_BASE_URL is not set in the environment.");
  }

  return (
    <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
      <div className="min-h-screen flex flex-col items-center justify-start px-4 pt-24">
        <div className="absolute top-4 right-4">
          <ModeToggle />
        </div>
        <h1 className="text-4xl font-bold mb-6 text-center">Scanza</h1>
        <div className="w-full max-w-md">
          <AddressInput onSubmit={(addr) => setAddress(addr)} />
          {address && (
            <div className="mt-6">
              <BalanceDisplay address={address} baseUrl={apiBaseUrl} />
            </div>
          )}
        </div>
      </div>
    </ThemeProvider>
  );
}

export default App;
