import { formatUnits } from "ethers";
import { TableCell, TableRow } from "@/components/ui/table";
import {
	formatAmount,
	formatFunctionName,
	normalizeString,
	timeAgo,
} from "@/lib/txUtils";
import type { Transaction } from "@/lib/types";
import { AddressCell } from "./AddressCell";
import { MethodCell } from "./MethodCell";
import { TransactionHashCell } from "./TransactionHashCell";

interface TransactionRowProps {
	tx: Transaction;
	address: string;
}

export default function TransactionRow({ tx, address }: TransactionRowProps) {
	const isIncoming = normalizeString(tx.to) === normalizeString(address);
	const valueEth = parseFloat(formatUnits(tx.value, 18));
	const gasFee = parseFloat(
		formatUnits((BigInt(tx.gasUsed) * BigInt(tx.gasPrice)).toString(), 18),
	);
	const rawFunctionName = tx.functionName
		? normalizeString(tx.functionName)
		: "";
	const method = rawFunctionName
		? formatFunctionName(rawFunctionName)
		: "Transfer";

	return (
		<TableRow>
			<TableCell className="font-mono flex items-center gap-1">
				<TransactionHashCell hash={tx.hash} />
			</TableCell>
			<TableCell className="max-w-[80px] overflow-hidden whitespace-nowrap text-ellipsis">
				<MethodCell method={method} />
			</TableCell>
			<TableCell>{BigInt(tx.blockNumber)}</TableCell>
			<TableCell>{timeAgo(tx.timeStamp)}</TableCell>
			<TableCell className="font-mono">
				<AddressCell address={tx.from} />
			</TableCell>
			<TableCell className="font-mono">
				<AddressCell address={tx.to} />
				{isIncoming ? (
					<span className="ml-2 px-1 text-xs rounded bg-green-100 text-green-700">
						IN
					</span>
				) : (
					<span className="ml-2 px-1 text-xs rounded bg-yellow-100 text-yellow-700">
						OUT
					</span>
				)}
			</TableCell>
			<TableCell className="text-right">{`${formatAmount(valueEth)} ETH`}</TableCell>
			<TableCell className="text-right">{formatAmount(gasFee)}</TableCell>
		</TableRow>
	);
}
