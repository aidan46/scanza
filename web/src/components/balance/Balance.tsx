import { Loader2 } from "lucide-react";
import { useEffect, useState } from "react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import type { SummaryResponse } from "@/lib/types";
import Pagination from "../Pagination";
import BalanceTable from "./BalanceTable";

interface BalanceProps {
  address: string;
  baseUrl: string;
  chain: string;
}

const TOKENS_PER_PAGE = 5;

export default function Balance({ address, baseUrl, chain }: BalanceProps) {
  const [data, setData] = useState<SummaryResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(0);

  useEffect(() => {
    setLoading(true);
    setError(null);
    setData(null);
    setPage(0);

    const fetchSummary = async () => {
      try {
        const res = await fetch(`${baseUrl}/${chain}/wallet/${address}`);
        if (!res.ok) {
          const errorText = await res.text();
          throw new Error(`HTTP ${res.status}: ${errorText}`);
        }

        const summary: SummaryResponse = await res.json();
        setData(summary);
      } catch (err) {
        setError(`Failed to fetch summary: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    fetchSummary();
  }, [address, baseUrl, chain]);

  if (loading) {
    return (
      <div className="mt-4 w-full max-w-2xl mx-auto text-center space-y-4 animate-fade-in">
        <div className="flex justify-center">
          <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
        </div>
      </div>
    );
  }

  if (error || !data) {
    return (
      <Alert variant="destructive" className="mt-4 w-full max-w-md mx-auto">
        <AlertTitle>Error</AlertTitle>
        <AlertDescription>{error}</AlertDescription>
      </Alert>
    );
  }

  const allTokens = [
    {
      token: {
        name: "Ethereum",
        address: "native",
        symbol: "ETH",
        decimals: 18,
      },
      balance: data.native_balance,
    },
    ...data.tokens,
  ];

  const start = page * TOKENS_PER_PAGE;
  const paginatedTokens = allTokens.slice(start, start + TOKENS_PER_PAGE);
  const totalPages = Math.ceil(allTokens.length / TOKENS_PER_PAGE);

  return (
    <div className="overflow-x-auto">
      <BalanceTable tokens={paginatedTokens} />
      {totalPages > 1 && (
        <Pagination page={page} totalPages={totalPages} setPage={setPage} />
      )}
    </div>
  );
}
