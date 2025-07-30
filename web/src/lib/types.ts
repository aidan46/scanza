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

export type Address = string;
export type Hash = string;

export type Transaction = {
	hash: Hash;
	from: Address;
	to: Address;
	value: string;
	gasPrice: string;
	gasUsed: string;
	timeStamp: string;
	blockNumber: string;
	input: string;
};
