import { isAddress } from "ethers";
import { useEffect, useState } from "react";
import { useLocation, useParams } from "react-router-dom";
import Balance from "@/components/balance/Balance";
import Transactions from "@/components/transactions/Transactions";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

interface WalletPageProps {
  baseUrl: string;
}

export default function WalletPage({ baseUrl }: WalletPageProps) {
  const { address } = useParams<{ address: string }>();
  const location = useLocation();
  const [tab, setTab] = useState("overview");

  // Sync tab with URL hash
  useEffect(() => {
    if (location.hash === "#transactions") setTab("transactions");
    else setTab("overview");
  }, [location.hash]);

  const handleTabChange = (value: string) => {
    setTab(value);
    window.history.replaceState(null, "", `#${value}`);
  };

  if (!isAddress(address)) {
    return (
      <p className="text-center mt-8 text-red-500 font-medium">
        Invalid Ethereum address
      </p>
    );
  }

  return (
    <div className="px-4 py-8">
      <div className="max-w-screen-xl mx-auto">
        <Tabs value={tab} onValueChange={handleTabChange} className="w-full">
          <div className="w-full flex justify-center mb-6">
            <TabsList className="bg-muted rounded-lg p-1 shadow-md">
              <TabsTrigger value="overview">Overview</TabsTrigger>
              <TabsTrigger value="transactions">Transactions</TabsTrigger>
            </TabsList>
          </div>

          <TabsContent value="overview">
            <div className="w-full flex justify-center">
              <div className="w-full max-w-2xl">
                <Balance address={address} baseUrl={baseUrl} chain="ethereum" />
              </div>
            </div>
          </TabsContent>

          <TabsContent value="transactions">
            <div className="w-full flex justify-center">
              <div className="w-full max-w-5xl">
                <Transactions
                  address={address}
                  baseUrl={baseUrl}
                  chain="ethereum"
                />
              </div>
            </div>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  );
}
