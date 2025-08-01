import { Button } from "./ui/button";

interface PaginationProps {
	page: number;
	totalPages: number;
	setPage: (page: number) => void;
	includeNext?: boolean;
}

export default function Pagination({
	page,
	totalPages,
	setPage,
	includeNext = true,
}: PaginationProps) {
	return (
		<div className="flex justify-between items-center mt-4">
			{page > 0 ? (
				<Button variant="outline" size="sm" onClick={() => setPage(page - 1)}>
					Previous
				</Button>
			) : (
				<div />
			)}

			<span className="text-sm text-muted-foreground">
				Page {page + 1}
				{includeNext && ` of ${totalPages}`}
			</span>

			{page + 1 < totalPages ? (
				<Button variant="outline" size="sm" onClick={() => setPage(page + 1)}>
					Next
				</Button>
			) : (
				<div />
			)}
		</div>
	);
}
