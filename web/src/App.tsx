import { Route, Routes } from "react-router-dom";
import Header from "./components/Header";
import { ThemeProvider } from "./components/theme/ThemeProvider";
import LandingPage from "./pages/LandingPage";
import WalletPage from "./pages/WalletPage";

const apiBaseUrl = import.meta.env.VITE_API_BASE_URL;

if (!apiBaseUrl) {
  throw new Error("VITE_API_BASE_URL is not set in the environment.");
}

export default function App() {
  return (
    <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
      <Header />
      <Routes>
        <Route path="/" element={<LandingPage />} />
        <Route
          path="/wallet/:address"
          element={<WalletPage baseUrl={apiBaseUrl} />}
        />
      </Routes>
    </ThemeProvider>
  );
}
