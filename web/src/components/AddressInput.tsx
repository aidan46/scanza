import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

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
    <form
      onSubmit={handleSubmit}
      className="w-full max-w-md space-y-3 min-h-[110px] text-center"
    >
      <Input
        type="text"
        value={input}
        onChange={(e) => setInput(e.target.value)}
        placeholder="Enter Ethereum address"
      />
      <Button type="submit" className="w-full">
        Submit
      </Button>
      <p className="text-sm h-5 text-red-500 font-medium">
        {error || "\u00A0"}
      </p>
    </form>
  );
}
