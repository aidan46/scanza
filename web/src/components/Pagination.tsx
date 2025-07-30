import { Button } from "./ui/button";

interface PaginationProps {
	page: number;
	totalPages: number;
	setPage: (page: number) => void;
}

export default function Pagination({
	page,
	totalPages,
	setPage,
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
				Page {page + 1} of {totalPages}
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
