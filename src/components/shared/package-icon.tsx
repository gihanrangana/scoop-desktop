import type React from "react";
import { useEffect, useState } from "react";
import { Skeleton } from "@/components/ui/skeleton";
import { usePackageIconSources } from "@/hooks/use-package-icon-sources";
import { getLetterFallback } from "@/lib/package-icon";
import { cn } from "@/lib/utils";

interface PackageIconProps {
	name: string;
	homepage?: string | null;
	tryNative?: boolean;
	className?: string;
}

const PackageIcon: React.FC<PackageIconProps> = (props) => {
	const { name, homepage, tryNative, className } = props;

	const [index, setIndex] = useState(0);
	const [imageLoaded, setImageLoaded] = useState(false);

	const { sources, isResolving } = usePackageIconSources({
		name,
		homepage: homepage ?? null,
		tryNative: !!tryNative,
	});

	const src = sources[index];

	const goToNextSource = () =>
		setIndex((current) =>
			current + 1 < sources.length ? current + 1 : sources.length,
		);

	// biome-ignore lint/correctness/useExhaustiveDependencies: reset index when props change
	useEffect(() => setIndex(0), [name, homepage, tryNative]);

	// biome-ignore lint/correctness/useExhaustiveDependencies: reset image loaded state when src changes
	useEffect(() => setImageLoaded(false), [src]);

	const skeleton = (
		<Skeleton
			className={cn("size-8 shrink-0 rounded-md", className)}
			aria-hidden
		/>
	);

	if (isResolving || (sources.length === 0 && tryNative)) return skeleton;

	// All sources failed → letter fallback
	if (!src) {
		return (
			<div
				className={cn(
					"flex size-8 shrink-0 items-center justify-center rounded-md bg-muted font-medium text-muted-foreground text-sm",
					className,
				)}
				aria-hidden
			>
				{getLetterFallback(name)}
			</div>
		);
	}

	return (
		<div className={cn("relative size-8 shrink-0", className)}>
			{!imageLoaded && skeleton}

			<img
				src={src}
				alt=""
				className={cn(
					"size-full rounded-md object-contain transition-opacity",
					imageLoaded ? "opacity-100" : "opacity-0",
				)}
				onError={goToNextSource}
				onLoad={(e) => {
					const img = e.currentTarget;
					if (img.naturalWidth <= 16 || img.naturalHeight <= 16) {
						goToNextSource();
						return;
					}
					setImageLoaded(true);
				}}
			/>
		</div>
	);
};

export default PackageIcon;
