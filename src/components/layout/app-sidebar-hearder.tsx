import type React from "react";
import { Card, CardContent } from "@/components/ui/card";
import {
	SidebarHeader,
	SidebarMenu,
	SidebarMenuItem,
} from "@/components/ui/sidebar";

type AppSidebarHeaderProps = React.ComponentProps<typeof SidebarHeader>;

const AppSidebarHeader: React.FC<AppSidebarHeaderProps> = (props) => {
	return (
		<SidebarHeader {...props}>
			<SidebarMenu className="gap-4">
				<SidebarMenuItem className="flex flex-1 flex-col items-center justify-center text-center">
					<div className="flex items-center gap-2">
						<h1 className="text-center font-black text-2xl uppercase italic tracking-tighter md:text-3xl">
							scoop <span className="text-primary not-italic">GUI</span>
						</h1>
						<div className="mt1 flex items-center gap-1.5 rounded border border-zinc-200 bg-zinc-200/50 px-1 py-1.5 leading-0 dark:border-zinc-700 dark:bg-zinc-800">
							<span className="font-black text-[10px] text-zinc-400 tracking-wider">
								v1.0.0
							</span>
							<div className="h-1.5 w-1.5 animate-pulse rounded-full bg-green-500 shadow shadow-emerald-300" />
						</div>
					</div>
					<p className="mt-1 ml-0.5 font-extrabold text-[10px] text-zinc-500 uppercase tracking-wide">
						package manager for Windows
					</p>
				</SidebarMenuItem>
				<SidebarMenuItem>
					<Card className="shadow-lg shadow-zinc-500/10 [--card-spacing:--spacing(2)] dark:shadow-zinc-800/20">
						<CardContent className="flex flex-row items-center justify-between">
							<div>
								<div className="flex flex-row items-center justify-start gap-2">
									<span className="font-bold text-[11px] text-zinc-800 uppercase leading-normal">
										Scoop Engine
									</span>

									<div className="h-1.5 w-1.5 animate-pulse rounded-full bg-green-500 shadow shadow-emerald-300" />
								</div>
								<h2 className="animate-pulse font-black text-[16px] text-green-500 text-shadow-card uppercase leading-normal">
									scoop ready
								</h2>
							</div>

							<div className="mt1 flex items-center gap-1.5 rounded border border-zinc-200 bg-zinc-200/50 px-1 leading-4 dark:border-zinc-700 dark:bg-zinc-800">
								<span className="font-black text-[10px] text-zinc-400 tracking-wider">
									v1.0.0
								</span>
							</div>
						</CardContent>
					</Card>
				</SidebarMenuItem>
			</SidebarMenu>
		</SidebarHeader>
	);
};

export default AppSidebarHeader;
