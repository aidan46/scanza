import { Loader2 } from "lucide-react";
import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";
import Pagination from "@/components/Pagination";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { Address, Transaction } from "@/lib/types";
import TransactionTable from "./TransactionTable";

type ApiResponse = {
  address: Address;
  transactions: Transaction[];
  pagination: {
    page: number;
    offset: number;
    has_more: boolean;
    next_page: number | null;
  };
};

interface TransactionsProps {
  address: Address;
  baseUrl: string;
  chain: string;
}

const TXS_PER_PAGE = 10;

export default function Transactions({
  address,
  baseUrl,
  chain,
}: TransactionsProps) {
  const [data, setData] = useState<ApiResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  const [searchParams, setSearchParams] = useSearchParams();
  const pageParam = parseInt(searchParams.get("page") || "1", 10);
  const page = Number.isNaN(pageParam) || pageParam < 1 ? 1 : pageParam;

  useEffect(() => {
    setLoading(true);
    setError(null);
    setData(null);

    const fetchTxs = async () => {
      try {
        const res = await fetch(
          `${baseUrl}/${chain}/wallet/${address}/transactions?page=${page}&offset=${TXS_PER_PAGE}`,
        );
        if (!res.ok) {
          const text = await res.text();
          throw new Error(`HTTP ${res.status}: ${text}`);
        }

        const json = await res.json();
        setData(json);
      } catch (err) {
        setError(`Failed to fetch transactions: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    fetchTxs();
  }, [address, baseUrl, chain, page]);

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

  return (
    <Card className="mt-6 w-full shadow-lg">
      <CardHeader className="text-center space-y-1">
        <CardTitle className="text-lg font-semibold">Transactions</CardTitle>
        <p className="text-sm text-muted-foreground font-mono break-all">
          {address}
        </p>
      </CardHeader>
      <CardContent className="overflow-x-auto">
        {data.transactions.length === 0 ? (
          <div className="text-center text-muted-foreground py-4">
            No transactions found
          </div>
        ) : (
          <>
            <TransactionTable
              transactions={data.transactions}
              address={address}
            />
            {data.pagination.has_more && (
              <Pagination
                page={page - 1}
                totalPages={page + 1}
                setPage={(newPage) =>
                  setSearchParams({ page: String(newPage + 1) })
                }
              />
            )}
          </>
        )}
      </CardContent>
    </Card>
  );
}
