import {
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger,
} from "../ui/tooltip";

export function MethodCell({ method }: { method: string }) {
	const display = method.length > 10 ? `${method.slice(0, 10)}…` : method;

	return (
		<div className="max-w-[80px] overflow-hidden whitespace-nowrap text-ellipsis">
			<TooltipProvider>
				<Tooltip>
					<TooltipTrigger asChild>
						<span className="block truncate cursor-default">{display}</span>
					</TooltipTrigger>
					<TooltipContent
						side="top"
						className="font-mono text-xs max-w-sm break-words"
					>
						{method}
					</TooltipContent>
				</Tooltip>
			</TooltipProvider>
		</div>
	);
}
