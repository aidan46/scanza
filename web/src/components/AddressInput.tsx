import { isAddress } from "ethers";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

interface AddressInputProps {
	onSubmit: (address: string) => void;
	formClassName?: string;
	buttonClassName?: string;
	errorClassName?: string;
}

export default function AddressInput({
	onSubmit,
	formClassName = "",
	buttonClassName = "",
	errorClassName = "",
}: AddressInputProps) {
	const [input, setInput] = useState("");
	const [error, setError] = useState("");

	const handleSubmit = (e: React.FormEvent) => {
		e.preventDefault();
		if (!isAddress(input)) {
			setError("Invalid Ethereum address");
			return;
		}
		setError("");
		onSubmit(input);
		setInput("");
	};

	return (
		<form onSubmit={handleSubmit} className={formClassName}>
			<Input
				type="text"
				value={input}
				onChange={(e) => setInput(e.target.value)}
				placeholder="Enter Ethereum address"
			/>
			<Button type="submit" className={buttonClassName}>
				Go
			</Button>
			<p className={errorClassName}>{error || "\u00A0"}</p>
		</form>
	);
}
