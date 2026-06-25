import type { AppNavRouteName } from "@/navigation";

export interface SidebarItem {
	icon: React.ReactNode;
	label: string;
	route: AppNavRouteName;
}

export type UsePackageIconSources = {
	name: string;
	homepage?: string;
	tryNative: boolean;
};
