import {
	Table,
	TableBody,
	TableHead,
	TableHeader,
	TableRow,
} from "@/components/ui/table";
import type { Transaction } from "@/lib/types";
import TransactionRow from "./TransactionRow";

interface TransactionTableProps {
	transactions: Transaction[];
	address: string;
}

export default function TransactionTable({
	transactions,
	address,
}: TransactionTableProps) {
	return (
		<Table className="min-w-full text-sm">
			<TableHeader>
				<TableRow>
					<TableHead>Txn Hash</TableHead>
					<TableHead>Method</TableHead>
					<TableHead>Block</TableHead>
					<TableHead>Age</TableHead>
					<TableHead>From</TableHead>
					<TableHead>To</TableHead>
					<TableHead className="text-right">Amount (ETH)</TableHead>
					<TableHead className="text-right">Txn Fee</TableHead>
				</TableRow>
			</TableHeader>
			<TableBody>
				{transactions.map((tx) => (
					<TransactionRow key={tx.hash} tx={tx} address={address} />
				))}
			</TableBody>
		</Table>
	);
}
