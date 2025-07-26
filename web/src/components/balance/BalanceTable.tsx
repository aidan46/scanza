import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from "@/components/ui/table";
import type { Token } from "@/lib/types";

interface BalanceTableProps {
	tokens: Token[];
}

export default function BalanceTable({ tokens }: BalanceTableProps) {
	return (
		<Table className="min-w-full">
			<TableHeader>
				<TableRow>
					<TableHead>Name</TableHead>
					<TableHead>Symbol</TableHead>
					<TableHead className="text-right">Balance</TableHead>
				</TableRow>
			</TableHeader>
			<TableBody>
				{tokens.map(({ token, balance }) => {
					const raw = BigInt(balance);
					const value = Number(raw) / 10 ** token.decimals;
					return (
						<TableRow key={token.address}>
							<TableCell>{token.name}</TableCell>
							<TableCell>{token.symbol}</TableCell>
							<TableCell className="text-right">
								{value.toLocaleString("en-US", {
									minimumFractionDigits: 0,
									maximumFractionDigits: 4,
								})}
							</TableCell>
						</TableRow>
					);
				})}
			</TableBody>
		</Table>
	);
}
