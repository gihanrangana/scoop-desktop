import { HelpCircleIcon, SettingsIcon } from "lucide-react";
import type React from "react";
import AppSidebarHeader from "@/components/layout/app-sidebar-hearder";
import {
	Sidebar,
	SidebarContent,
	SidebarFooter,
	SidebarMenu,
	SidebarMenuButton,
	SidebarMenuItem,
} from "@/components/ui/sidebar";
import { useNavigation } from "@/navigation";
import type { SidebarItem } from "@/types";

type AppSidebarProps = React.ComponentProps<typeof Sidebar> & {
	sidebarItems: SidebarItem[];
};

const AppSidebar: React.FC<AppSidebarProps> = (props) => {
	const { sidebarItems, ...rest } = props;

	const { navigate, isActive } = useNavigation();

	return (
		<Sidebar collapsible="icon" {...rest}>
			<AppSidebarHeader />
			<SidebarContent className="mt-4 px-2">
				{sidebarItems.map((i) => (
					<SidebarMenu key={i.route}>
						<SidebarMenuItem>
							<SidebarMenuButton
								onClick={() => navigate(i.route)}
								isActive={isActive(i.route)}
							>
								{i.icon}
								<span>{i.label}</span>
							</SidebarMenuButton>
						</SidebarMenuItem>
					</SidebarMenu>
				))}
			</SidebarContent>

			<SidebarFooter className="gap-0">
				<SidebarMenuButton>
					<SettingsIcon />
					<span>Settings</span>
				</SidebarMenuButton>

				<SidebarMenuButton>
					<HelpCircleIcon />
					<span>Help</span>
				</SidebarMenuButton>
			</SidebarFooter>
		</Sidebar>
	);
};

export default AppSidebar;
