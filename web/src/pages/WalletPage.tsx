import { isAddress } from "ethers";
import { useEffect, useState } from "react";
import { useLocation, useParams } from "react-router-dom";
import { useChains } from "@/ChainsProvider";
import Balance from "@/components/balance/Balance";
import Transactions from "@/components/transactions/Transactions";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@/components/ui/select";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

interface WalletPageProps {
	baseUrl: string;
}

export default function WalletPage({ baseUrl }: WalletPageProps) {
	const { address } = useParams<{ address: string }>();
	const location = useLocation();
	const { chains, loading, error } = useChains();
	const [tab, setTab] = useState("overview");
	const [chain, setChain] = useState("eth");

	useEffect(() => {
		if (location.hash === "#transactions") setTab("transactions");
		else setTab("overview");
	}, [location.hash]);

	const handleTabChange = (value: string) => {
		setTab(value);
		window.history.replaceState(null, "", `#${value}`);
	};

	if (!isAddress(address)) {
		return (
			<p className="text-center mt-8 text-red-500 font-medium">
				Invalid address
			</p>
		);
	}

	if (loading) {
		return (
			<p className="text-center mt-8 text-muted-foreground font-medium">
				Loading chains...
			</p>
		);
	}

	if (error || chains.length === 0) {
		return (
			<p className="text-center mt-8 text-red-500 font-medium">
				Failed to load chains
			</p>
		);
	}

	return (
		<div className="px-4 py-8">
			<div className="max-w-screen-xl mx-auto">
				{/* Chain selector */}
				<div className="flex justify-end mb-6">
					<Select value={chain} onValueChange={setChain}>
						<SelectTrigger className="w-[180px]">
							<SelectValue placeholder="Select chain" />
						</SelectTrigger>
						<SelectContent>
							{chains.map((chain) => {
								if (!chain || !chain.shortName) return null;

								return (
									<SelectItem key={chain.shortName} value={chain.shortName}>
										{chain.name}
									</SelectItem>
								);
							})}
						</SelectContent>
					</Select>
				</div>

				{/* Unified Card with Tabs */}
				<Tabs value={tab} onValueChange={handleTabChange}>
					<Card className="w-full max-w-7xl mx-auto shadow-lg">
						<CardHeader className="flex flex-col md:flex-row md:items-center md:justify-between gap-4">
							<CardTitle className="text-lg font-semibold">
								Wallet Details
							</CardTitle>
							<TabsList className="bg-muted rounded-lg p-1 shadow-md">
								<TabsTrigger value="overview">Overview</TabsTrigger>
								<TabsTrigger value="transactions">Transactions</TabsTrigger>
							</TabsList>
						</CardHeader>

						<TabsContent value="overview">
							<CardContent>
								<Balance address={address} baseUrl={baseUrl} chain={chain} />
							</CardContent>
						</TabsContent>
						<TabsContent value="transactions">
							<CardContent>
								<Transactions
									address={address}
									baseUrl={baseUrl}
									chain={chain}
								/>
							</CardContent>
						</TabsContent>
					</Card>
				</Tabs>
			</div>
		</div>
	);
}
