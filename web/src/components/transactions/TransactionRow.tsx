import { formatUnits } from "ethers";
import { TableCell, TableRow } from "@/components/ui/table";
import { normalizeHex, shortHash, timeAgo } from "@/lib/txUtils";
import type { Transaction } from "@/lib/types";

interface TransactionRowProps {
	tx: Transaction;
	address: string;
}

export default function TransactionRow({ tx, address }: TransactionRowProps) {
	const isIncoming = normalizeHex(tx.to) === normalizeHex(address);
	const valueEth = parseFloat(formatUnits(tx.value, 18));
	const gasFee = parseFloat(
		formatUnits((BigInt(tx.gasUsed) * BigInt(tx.gasPrice)).toString(), 18),
	);
	const method = tx.input && tx.input !== "0x" ? "Contract Call" : "Transfer";

	return (
		<TableRow>
			<TableCell className="font-mono">{shortHash(tx.hash)}</TableCell>
			<TableCell>{method}</TableCell>
			<TableCell>{tx.blockNumber}</TableCell>
			<TableCell>{timeAgo(tx.timeStamp)}</TableCell>
			<TableCell className="font-mono">{shortHash(tx.from)}</TableCell>
			<TableCell className="font-mono">
				{shortHash(tx.to)}
				{isIncoming && (
					<span className="ml-1 px-1 text-xs rounded bg-green-100 text-green-700">
						IN
					</span>
				)}
			</TableCell>
			<TableCell className="text-right">{valueEth.toFixed(5)}</TableCell>
			<TableCell className="text-right">{gasFee.toFixed(6)}</TableCell>
		</TableRow>
	);
}
