import { useState } from "react";

type AddressInputProps = {
	onSubmit: (address: string) => void;
};

export default function AddressInput({ onSubmit }: AddressInputProps) {
	const [input, setInput] = useState("");
	const [error, setError] = useState("");

	const isValidEthAddress = (address: string): boolean =>
		/^0x[a-fA-F0-9]{40}$/.test(address);

	const handleSubmit = (e: React.FormEvent) => {
		e.preventDefault();
		if (!isValidEthAddress(input)) {
			setError("Invalid Ethereum address");
			return;
		}
		setError("");
		onSubmit(input);
	};

	return (
		<form onSubmit={handleSubmit}>
			<input
				type="text"
				value={input}
				onChange={(e) => setInput(e.target.value)}
				placeholder="Enter Ethereum address"
			/>
			<button type="submit">Submit</button>
			{error && <p style={{ color: "red" }}>{error}</p>}
		</form>
	);
}
