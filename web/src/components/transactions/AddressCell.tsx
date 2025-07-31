import { CopyableText } from "@/components/CopyableText";
import { normalizeString, shortAddress } from "@/lib/txUtils";

interface AddressCellProps {
  address: string;
}

export function AddressCell({ address }: AddressCellProps) {
  return (
    <CopyableText
      fullText={normalizeString(address)}
      displayText={shortAddress(address)}
      fullTextTooltip={true}
      copyTooltipLabel="Copy Address"
    />
  );
}
