import { Check, Copy } from "lucide-react";
import { useState } from "react";
import {
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger,
} from "@/components/ui/tooltip";

interface CopyableTextProps {
	text: string;
	display: React.ReactNode;
	tooltipLabel?: string;
	iconSize?: number;
	className?: string;
}

export function CopyableText({
	text,
	display,
	tooltipLabel = "Copy",
	iconSize = 14,
	className = "",
}: CopyableTextProps) {
	const [copied, setCopied] = useState(false);

	const handleCopy = () => {
		navigator.clipboard.writeText(text);
		setCopied(true);
		setTimeout(() => setCopied(false), 1500);
	};

	return (
		<span className={`inline-flex items-center gap-1 ${className}`}>
			<span className="cursor-default">{display}</span>
			<TooltipProvider>
				<Tooltip open={copied || undefined}>
					<TooltipTrigger asChild>
						<button
							type="button"
							onClick={handleCopy}
							className="p-0 bg-transparent border-none cursor-pointer hover:text-muted-foreground"
						>
							{copied ? (
								<Check
									className="text-green-600"
									width={iconSize}
									height={iconSize}
								/>
							) : (
								<Copy width={iconSize} height={iconSize} />
							)}
						</button>
					</TooltipTrigger>
					<TooltipContent side="top">
						{copied ? "Copied!" : tooltipLabel}
					</TooltipContent>
				</Tooltip>
			</TooltipProvider>
		</span>
	);
}
