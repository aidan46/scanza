import { useNavigate } from "react-router-dom";
import AddressInput from "@/components/AddressInput";

export default function LandingPage() {
  const navigate = useNavigate();

  return (
    <div className="min-h-screen px-4 pt-[25vh] flex flex-col items-center text-center">
      <p className="text-lg mb-6">
        View balances for any Ethereum wallet address.
      </p>
      <div className="w-full max-w-md min-h-[110px]">
        <AddressInput
          onSubmit={(addr) => navigate(`/wallet/${addr}`)}
          formClassName="space-y-3"
          buttonClassName="w-full"
          errorClassName="text-sm h-5 text-red-500 font-medium"
        />
      </div>
    </div>
  );
}
