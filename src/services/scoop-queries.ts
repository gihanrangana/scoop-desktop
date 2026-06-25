import { queryOptions } from "@tanstack/react-query";
import {
	checkIfInstalled,
	getAllPackages,
	getBuckets,
	getInstalledPackages,
	getPackageIcon,
	getPackageInfo,
	getPackagesByBucket,
	getRecentInstalledPackages,
	getScoopStats,
	searchPackages,
} from "@/services/scoopApi";

const SCOPE = "scoop";

type QueryExtras = {
	enabled?: boolean;
	staleTime?: number;
	gcTime?: number;
};

const defineQuery = <TArgs extends unknown[], TData>(
	name: string,
	fn: (...args: TArgs) => Promise<TData>,
	extras?: (...args: TArgs) => QueryExtras,
) => {
	return (...args: TArgs) =>
		queryOptions({
			queryKey: [SCOPE, name, ...args],
			queryFn: () => fn(...args),
			...(extras?.(...args) || {}),
		});
};

export const scoopQueries = {
	stats: defineQuery("stats", getScoopStats),
	installed: defineQuery("installed", getInstalledPackages),
	recentInstalled: defineQuery("recentInstalled", (limit: number = 5) =>
		getRecentInstalledPackages(limit),
	),
	buckets: defineQuery("buckets", getBuckets),
	packages: defineQuery("packages", getAllPackages),
	packagesByBucket: defineQuery("packagesByBucket", getPackagesByBucket),
	search: defineQuery("search", searchPackages, (query) => ({
		enabled: query.length > 0,
	})),
	checkInstalled: defineQuery("checkInstalled", checkIfInstalled),
	packageInfo: defineQuery("packageInfo", getPackageInfo, (name, bucket) => ({
		enabled: name.length > 0 && bucket.length > 0,
	})),
	packageIcon: defineQuery(
		"packageIcon",
		(name: string, homepage?: string) => getPackageIcon(name, homepage),
		(name, _homepage) => ({
			enabled: name.length > 0,
			staleTime: Number.POSITIVE_INFINITY,
			retry: false,
		}),
	),
};
