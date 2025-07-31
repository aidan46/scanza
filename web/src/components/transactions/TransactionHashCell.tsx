import { CopyableText } from "@/components/CopyableText";
import { normalizeString, shortHash } from "@/lib/txUtils";

export function TransactionHashCell({ hash }: { hash: string }) {
	return (
		<div className="font-mono">
			<CopyableText
				text={normalizeString(hash)}
				display={shortHash(hash)}
				tooltipLabel="Copy Transaction Hash"
				iconSize={16}
			/>
		</div>
	);
}
