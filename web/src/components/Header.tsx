import { useLocation, useNavigate } from "react-router-dom";
import AddressInput from "./AddressInput";
import { ModeToggle } from "./theme/ModeToggle";
import { Button } from "./ui/button";

export default function Header() {
	const location = useLocation();
	const navigate = useNavigate();

	const isWalletPage = location.pathname.startsWith("/wallet");

	return (
		<header className="w-full px-4 py-2 flex items-center justify-between gap-4 flex-wrap">
			<Button type="button" variant="ghost" onClick={() => navigate("/")}>
				Scanza
			</Button>

			<div className="flex items-center gap-2 ml-auto">
				{isWalletPage && (
					<AddressInput
						onSubmit={(addr) => navigate(`/wallet/${addr}`)}
						formClassName="flex gap-2 items-center"
						buttonClassName=""
						errorClassName="text-sm text-red-500"
					/>
				)}
				<ModeToggle />
			</div>
		</header>
	);
}
