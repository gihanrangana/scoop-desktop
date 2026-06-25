import { DatabaseIcon, HomeIcon, Package2Icon, SearchIcon } from "lucide-react";
import AppShell from "@/components/layout/app-shell";
import ScreenOutlet from "@/components/layout/screen-outlet";
import { NavigationProvider } from "@/navigation";

function App() {
	return (
		<NavigationProvider initialRoute={{ name: "home" }}>
			{/* <main className="min-h-svh w-full"> */}
			<AppShell
				sidebarItems={[
					{
						icon: <HomeIcon />,
						label: "Home",
						route: "home",
					},
					{
						icon: <SearchIcon />,
						label: "Browse",
						route: "browse",
					},
					{
						icon: <Package2Icon />,
						label: "Installed",
						route: "installed",
					},
					{
						icon: <DatabaseIcon />,
						label: "Buckets",
						route: "buckets",
					},
				]}
			>
				<ScreenOutlet />
			</AppShell>
			{/* </main> */}
		</NavigationProvider>
	);
}

export default App;
