import type React from "react";
import AppHeader from "@/components/layout/app-header";
import AppSidebar from "@/components/layout/app-sidebar";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";
import type { SidebarItem } from "@/types";

interface AppShellProps {
	children: React.ReactNode;
	sidebarItems: SidebarItem[];
}

const AppShell: React.FC<AppShellProps> = (props) => {
	return (
		<SidebarProvider className="min-h-svh w-full">
			<AppSidebar variant="inset" sidebarItems={props.sidebarItems} />
			<SidebarInset className="max-h-full overflow-hidden">
				<div className="flex flex-1 flex-col">
					<div className="@container/main flex flex-1 flex-col gap-2">
						<AppHeader />

						<div className="flex flex-col gap-4 px-4 py-4 md:gap-6 md:py-6">
							{props.children}
						</div>
					</div>
				</div>
			</SidebarInset>
		</SidebarProvider>
	);
};

export default AppShell;
