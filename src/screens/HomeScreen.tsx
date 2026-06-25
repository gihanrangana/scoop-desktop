import { ArrowRight, ChevronRightIcon } from "lucide-react";
import type React from "react";
import StatCards from "@/components/layout/home/stat-cards";
import LoadingBlock from "@/components/shared/loading-block";
import PackageIcon from "@/components/shared/package-icon";
import { Button } from "@/components/ui/button";
import {
	Card,
	CardAction,
	CardContent,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";
import { useScoop } from "@/hooks/use-scoop";
import { scoopQueries } from "@/services/scoop-queries";

const HomeScreen: React.FC = () => {
	const [stats, recentInstalled] = useScoop([
		scoopQueries.stats(),
		scoopQueries.recentInstalled(5),
	]);

	if (stats.isLoading || recentInstalled.isLoading) return <LoadingBlock />;

	return (
		<div className="flex flex-col gap-4">
			<div className="grid grid-cols-3 gap-4">
				<StatCards
					installed={stats.data?.installed ?? 0}
					buckets={stats.data?.buckets ?? 0}
					available={stats.data?.packages ?? 0}
				/>
			</div>

			<Card className="shadow-lg shadow-zinc-500/10 [--card-spacing:--spacing(2)] dark:shadow-zinc-800/20">
				<CardHeader>
					<CardTitle className="font-semibold">Recently Installed</CardTitle>
					<CardAction>
						<Button variant="ghost" size="sm" className="text-primary">
							View All <ArrowRight size={18} />
						</Button>
					</CardAction>
				</CardHeader>

				<CardContent className="p-0">
					{recentInstalled.data?.map((pkg) => (
						<div
							key={pkg.name}
							className="flex items-center gap-3 px-2 py-1.5 hover:bg-muted"
						>
							<div className="flex h-full items-center">
								<PackageIcon
									name={pkg.name}
									homepage={pkg.homepage}
									tryNative={true}
								/>
							</div>
							<div className="flex-1">
								<p className="font-medium">{pkg.name}</p>
								<p className="line-clamp-1 text-muted-foreground text-xs">
									{pkg.description}
								</p>
							</div>
							<div className="flex h-full items-center text-foreground/30">
								<ChevronRightIcon size={18} />
							</div>
						</div>
					))}
				</CardContent>
			</Card>
		</div>
	);
};

export default HomeScreen;
