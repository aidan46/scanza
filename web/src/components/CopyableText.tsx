import { Check, Copy } from "lucide-react";
import { useState } from "react";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";

interface CopyableTextProps {
  fullText: string;
  displayText: string;
  fullTextTooltip?: boolean;
  copyTooltipLabel: string;
}

export function CopyableText({
  fullText,
  displayText,
  fullTextTooltip = false,
  copyTooltipLabel,
}: CopyableTextProps) {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(fullText);
    setCopied(true);
    setTimeout(() => setCopied(false), 1500);
  };

  return (
    <TooltipProvider>
      <span className={`inline-flex items-center gap-1`}>
        {/* Tooltip for the full text on hover */}
        {fullTextTooltip ? (
          <Tooltip>
            <TooltipTrigger asChild>
              <span className="cursor-default font-mono truncate hover:text-muted-foreground">
                {displayText}
              </span>
            </TooltipTrigger>
            <TooltipContent side="top">{fullText}</TooltipContent>
          </Tooltip>
        ) : (
          <span className="cursor-default font-mono truncate">
            {displayText}
          </span>
        )}

        {/* Tooltip for the copy icon */}
        <Tooltip open={copied || undefined}>
          <TooltipTrigger asChild>
            <button
              type="button"
              onClick={handleCopy}
              className="p-0 bg-transparent border-none cursor-pointer hover:text-muted-foreground ml-1"
              aria-label="Copy to clipboard"
            >
              {copied ? (
                <Check className="text-green-600" width={14} height={14} />
              ) : (
                <Copy width={14} height={14} />
              )}
            </button>
          </TooltipTrigger>
          <TooltipContent side="top">
            {copied ? "Copied!" : copyTooltipLabel}
          </TooltipContent>
        </Tooltip>
      </span>
    </TooltipProvider>
  );
}
