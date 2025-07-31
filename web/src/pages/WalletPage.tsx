import { isAddress } from "ethers";
import { useEffect, useState } from "react";
import { useLocation, useParams } from "react-router-dom";
import Balance from "@/components/balance/Balance";
import Transactions from "@/components/transactions/Transactions";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { SUPPORTED_CHAINS } from "@/lib/constants";

interface WalletPageProps {
  baseUrl: string;
}

export default function WalletPage({ baseUrl }: WalletPageProps) {
  const { address } = useParams<{ address: string }>();
  const location = useLocation();

  const [tab, setTab] = useState("overview");
  const [chain, setChain] = useState("ethereum");

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
        Invalid address
      </p>
    );
  }

  return (
    <div className="px-4 py-8">
      <div className="max-w-screen-xl mx-auto">
        {/* Chain selector */}
        <div className="flex justify-end mb-6">
          <Select value={chain} onValueChange={setChain}>
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Select chain" />
            </SelectTrigger>
            <SelectContent>
              {SUPPORTED_CHAINS.map(({ id, label }) => (
                <SelectItem key={id} value={id}>
                  {label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {/* Unified Card with Tabs */}
        <Tabs value={tab} onValueChange={handleTabChange}>
          <Card className="w-full max-w-5xl mx-auto shadow-lg">
            <CardHeader className="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
              <CardTitle className="text-lg font-semibold">
                Wallet Details
              </CardTitle>
              <TabsList className="bg-muted rounded-lg p-1 shadow-md">
                <TabsTrigger value="overview">Overview</TabsTrigger>
                <TabsTrigger value="transactions">Transactions</TabsTrigger>
              </TabsList>
            </CardHeader>

            <TabsContent value="overview">
              <CardContent>
                <Balance address={address} baseUrl={baseUrl} chain={chain} />
              </CardContent>
            </TabsContent>
            <TabsContent value="transactions">
              <CardContent>
                <Transactions
                  address={address}
                  baseUrl={baseUrl}
                  chain={chain}
                />
              </CardContent>
            </TabsContent>
          </Card>
        </Tabs>
      </div>
    </div>
  );
}
