import { Loader2 } from "lucide-react";
import { useEffect, useState } from "react";
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
}

const TXS_PER_PAGE = 10;

export default function Transactions({ address, baseUrl }: TransactionsProps) {
	const [data, setData] = useState<ApiResponse | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [loading, setLoading] = useState(true);
	const [page, setPage] = useState(0);

	useEffect(() => {
		setLoading(true);
		setError(null);
		setData(null);

		const fetchTxs = async () => {
			try {
				const res = await fetch(
					`${baseUrl}/wallet/${address}/transactions?page=${page + 1}&offset=${TXS_PER_PAGE}`,
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
	}, [address, baseUrl, page]);

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
							<Pagination page={page} totalPages={page + 2} setPage={setPage} />
						)}
					</>
				)}
			</CardContent>
		</Card>
	);
}
