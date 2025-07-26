export type Token = {
	token: {
		name: string;
		address: string;
		symbol: string;
		decimals: number;
	};
	balance: string;
};

export type SummaryResponse = {
	address: string;
	native_balance: string;
	tokens: Token[];
};
