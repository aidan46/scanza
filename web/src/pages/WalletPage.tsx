import { isAddress } from "ethers";
import { useParams } from "react-router-dom";
import Balance from "@/components/balance/Balance";
import Transactions from "@/components/transactions/Transactions";

interface WalletPageProps {
	baseUrl: string;
}

export default function WalletPage({ baseUrl }: WalletPageProps) {
	const { address } = useParams<{ address: string }>();

	if (!isAddress(address)) {
		return (
			<p className="text-center mt-8 text-red-500 font-medium">
				Invalid Ethereum address
			</p>
		);
	}

	return (
		<div className="px-4 py-8">
			<div className="mx-auto w-full max-w-2xl">
				<Balance address={address} baseUrl={baseUrl} />
			</div>
			<div className="mx-auto w-full max-w-2xl">
				<Transactions address={address} baseUrl={baseUrl} />
			</div>
		</div>
	);
}
