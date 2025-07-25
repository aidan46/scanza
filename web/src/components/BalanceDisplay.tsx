import { useEffect, useState } from "react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Skeleton } from "@/components/ui/skeleton";

type BalanceDisplayProps = {
  address: string;
  baseUrl: string;
};

type BalanceResponse = {
  address: string;
  balance: string;
};

export default function BalanceDisplay({
  address,
  baseUrl,
}: BalanceDisplayProps) {
  const [balance, setBalance] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const fetchBalance = async () => {
      try {
        const res = await fetch(`${baseUrl}/wallet/${address}/balance`);
        if (!res.ok) {
          const errorText = await res.text();
          throw new Error(`HTTP ${res.status}: ${errorText}`);
        }

        const data: BalanceResponse = await res.json();
        const raw = BigInt(data.balance);
        const formatted = (raw / 10n ** 18n).toString();
        setBalance(formatted);
      } catch (err) {
        setError(`Failed to fetch balance: ${err}`);
      } finally {
        setLoading(false);
      }
    };

    fetchBalance();
  }, [address, baseUrl]);

  if (loading) {
    return (
      <div className="mt-4 w-full max-w-md text-center space-y-2">
        <p className="text-muted-foreground text-sm">
          Looking up address: <span className="font-mono">{address}</span>
        </p>
        <Skeleton className="h-6 w-48 mx-auto" />
      </div>
    );
  }

  if (error) {
    return (
      <Alert variant="destructive" className="mt-4 w-full max-w-md mx-auto">
        <AlertTitle>Error</AlertTitle>
        <AlertDescription>{error}</AlertDescription>
      </Alert>
    );
  }

  return (
    <Card className="mt-6 w-full max-w-md shadow-lg">
      <CardHeader>
        <CardTitle className="text-lg font-semibold text-center">
          Balance
        </CardTitle>
      </CardHeader>
      <CardContent className="text-center">
        <p className="text-2xl font-bold tracking-tight">
          {Number(balance).toLocaleString("en-GB")}{" "}
          <span className="text-muted-foreground text-base">ETH</span>
        </p>
      </CardContent>
    </Card>
  );
}
