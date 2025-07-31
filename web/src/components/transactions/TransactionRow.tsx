import { formatUnits } from "ethers";
import { TableCell, TableRow } from "@/components/ui/table";
import {
	formatAmount,
	formatFunctionName,
	normalizeString,
	shortHash,
	timeAgo,
} from "@/lib/txUtils";
import type { Transaction } from "@/lib/types";
import {
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger,
} from "../ui/tooltip";

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
			<TableCell className="font-mono">{shortHash(tx.hash)}</TableCell>
			<TableCell className="max-w-[80px] overflow-hidden whitespace-nowrap text-ellipsis">
				<TooltipProvider>
					<Tooltip>
						<TooltipTrigger asChild>
							<span className="block truncate cursor-default">
								{method.length > 10 ? `${method.slice(0, 10)}…` : method}
							</span>
						</TooltipTrigger>
						<TooltipContent
							side="top"
							className="font-mono text-xs max-w-sm break-words"
						>
							{method}
						</TooltipContent>
					</Tooltip>
				</TooltipProvider>
			</TableCell>
			<TableCell>{BigInt(tx.blockNumber)}</TableCell>
			<TableCell>{timeAgo(tx.timeStamp)}</TableCell>
			<TableCell className="font-mono">
				<TooltipProvider>
					<Tooltip>
						<TooltipTrigger asChild>
							<span>{shortHash(tx.from)}</span>
						</TooltipTrigger>
						<TooltipContent side="top" className="font-mono text-xs">
							{normalizeString(tx.from)}
						</TooltipContent>
					</Tooltip>
				</TooltipProvider>
			</TableCell>
			<TableCell className="font-mono">
				<TooltipProvider>
					<Tooltip>
						<TooltipTrigger asChild>
							<span>{shortHash(tx.to)}</span>
						</TooltipTrigger>
						<TooltipContent side="top" className="font-mono text-xs">
							{normalizeString(tx.to)}
						</TooltipContent>
					</Tooltip>
				</TooltipProvider>
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
