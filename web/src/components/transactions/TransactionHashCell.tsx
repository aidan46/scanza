import { CopyableText } from "@/components/CopyableText";
import { normalizeString, shortHash } from "@/lib/txUtils";

export function TransactionHashCell({ hash }: { hash: string }) {
  return (
    <div className="font-mono">
      <CopyableText
        fullText={normalizeString(hash)}
        displayText={shortHash(hash)}
        copyTooltipLabel="Copy Transaciton Hash"
      />
    </div>
  );
}
