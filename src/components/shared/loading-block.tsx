import { Loader2Icon } from "lucide-react";
import type React from "react";

interface LoadingBlockProps {
	variant?: "default" | "inline";
	message?: string;
}

const LoadingBlock: React.FC<LoadingBlockProps> = ({
	variant = "default",
	message,
}) => {
	if (variant === "inline") {
		return (
			<div className="flex flex-row items-center justify-center gap-2 text-zinc-300">
				<Loader2Icon className="animate-spin" size={18} />
				<span className="text-sm">{message || "Loading data..."}</span>
			</div>
		);
	}

	return (
		<div className="flex h-full min-h-40 w-full flex-col items-center justify-center gap-2 text-zinc-300">
			<Loader2Icon className="animate-spin" size={48} />
			<span>Loading data...</span>
		</div>
	);
};

export default LoadingBlock;
