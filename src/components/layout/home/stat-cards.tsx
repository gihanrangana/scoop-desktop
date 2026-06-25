import { DatabaseIcon, Package2Icon, SearchIcon } from "lucide-react";
import type React from "react";
import {
	Card,
	CardAction,
	CardContent,
	CardHeader,
	CardTitle,
} from "@/components/ui/card";

interface StatCardsProps {
	installed: number;
	buckets: number;
	available: number;
}

const StatCards: React.FC<StatCardsProps> = (props) => {
	return (
		<>
			<Card className="shadow-lg shadow-zinc-500/10 [--card-spacing:--spacing(2)] dark:shadow-zinc-800/20">
				<CardHeader>
					<CardAction>
						<Package2Icon size={18} />
					</CardAction>
					<CardTitle>Installed</CardTitle>
				</CardHeader>

				<CardContent>
					<span className="font-black text-3xl">{props.installed}</span>
				</CardContent>
			</Card>

			<Card className="shadow-lg shadow-zinc-500/10 [--card-spacing:--spacing(2)] dark:shadow-zinc-800/20">
				<CardHeader>
					<CardAction>
						<DatabaseIcon size={18} />
					</CardAction>
					<CardTitle>Buckets</CardTitle>
				</CardHeader>

				<CardContent>
					<span className="font-black text-3xl">{props.buckets}</span>
				</CardContent>
			</Card>

			<Card className="shadow-lg shadow-zinc-500/10 [--card-spacing:--spacing(2)] dark:shadow-zinc-800/20">
				<CardHeader>
					<CardAction>
						<SearchIcon size={18} />
					</CardAction>
					<CardTitle>Available</CardTitle>
				</CardHeader>

				<CardContent>
					<span className="font-black text-3xl">
						{props.available.toLocaleString()}
					</span>
				</CardContent>
			</Card>
		</>
	);
};

export default StatCards;
