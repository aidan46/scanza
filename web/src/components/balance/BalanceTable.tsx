import { formatUnits } from "ethers";
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
	const nonZeroTokens = tokens.filter(({ balance }) => {
		try {
			return BigInt(balance) !== 0n;
		} catch {
			return false;
		}
	});

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
				{nonZeroTokens.length > 0 ? (
					nonZeroTokens.map(({ token, balance }) => {
						const formatted = formatUnits(balance, token.decimals);
						const formattedNumber = Number(formatted);
						return (
							<TableRow key={token.address}>
								<TableCell>{token.name}</TableCell>
								<TableCell>{token.symbol}</TableCell>
								<TableCell className="text-right">
									{formattedNumber.toLocaleString("en-US", {
										minimumFractionDigits: 0,
										maximumFractionDigits: 4,
									})}
								</TableCell>
							</TableRow>
						);
					})
				) : (
					<TableRow>
						<TableCell
							colSpan={3}
							className="text-center text-muted-foreground py-4"
						>
							No tokens found
						</TableCell>
					</TableRow>
				)}
			</TableBody>
		</Table>
	);
}
