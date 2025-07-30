import { isAddress } from "ethers";
import { useParams } from "react-router-dom";
import Balance from "@/components/balance/Balance";
import Transactions from "@/components/transactions/Transactions";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

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
			<div className="max-w-screen-xl mx-auto">
				<Tabs defaultValue="overview" className="w-full">
					{/* Tabs List centered + padded */}
					<div className="w-full flex justify-center mb-6">
						<TabsList className="bg-muted rounded-lg p-1 shadow-md">
							<TabsTrigger value="overview">Overview</TabsTrigger>
							<TabsTrigger value="transactions">Transactions</TabsTrigger>
						</TabsList>
					</div>

					{/* Overview Content */}
					<TabsContent value="overview">
						<div className="w-full flex justify-center">
							<div className="w-full max-w-2xl">
								<Balance address={address} baseUrl={baseUrl} />
							</div>
						</div>
					</TabsContent>

					{/* Transactions Content */}
					<TabsContent value="transactions">
						<div className="w-full flex justify-center">
							<div className="w-full max-w-5xl">
								<Transactions address={address} baseUrl={baseUrl} />
							</div>
						</div>
					</TabsContent>
				</Tabs>
			</div>
		</div>
	);
}
