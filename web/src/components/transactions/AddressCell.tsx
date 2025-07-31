import { CopyableText } from "@/components/CopyableText";
import { normalizeString, shortAddress } from "@/lib/txUtils";

interface AddressCellProps {
	address: string;
	className?: string;
}

export function AddressCell({ address, className = "" }: AddressCellProps) {
	const normalized = normalizeString(address);
	return (
		<CopyableText
			text={normalized}
			display={shortAddress(address)}
			tooltipLabel="Copy Address"
			className={className}
		/>
	);
}
