import { BellIcon, SearchIcon } from "lucide-react";
import type React from "react";
import { Button } from "@/components/ui/button";
// import { Input } from "@/components/ui/input";
import {
	InputGroup,
	InputGroupAddon,
	InputGroupInput,
} from "@/components/ui/input-group";

const AppHeader: React.FC = () => {
	return (
		<header className="flex h-12 w-full flex-row items-center justify-between px-4">
			<InputGroup className="max-w-sm">
				<InputGroupInput placeholder="Search packages (e.g. git, vscode, nodejs)..." />
				<InputGroupAddon>
					<SearchIcon />
				</InputGroupAddon>
			</InputGroup>
			<div>
				<Button variant="ghost" size="icon" className="rounded-full">
					<BellIcon />
				</Button>
			</div>
		</header>
	);
};

export default AppHeader;
