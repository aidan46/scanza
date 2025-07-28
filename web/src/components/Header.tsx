import { useNavigate } from "react-router-dom";
import { ModeToggle } from "./theme/ModeToggle";
import { Button } from "./ui/button";

export default function Header() {
  const navigate = useNavigate();

  return (
    <header className="w-full px-4 py-2 flex items-center justify-between">
      <Button type="button" variant="ghost" onClick={() => navigate("/")}>
        Scanza
      </Button>
      <ModeToggle />
    </header>
  );
}
