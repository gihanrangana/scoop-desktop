import { useQuery } from "@tanstack/react-query";
import { useMemo } from "react";
import { getRemoteIconUrls } from "@/lib/package-icon";
import { scoopQueries } from "@/services/scoop-queries";
import type { UsePackageIconSources } from "@/types";

export const usePackageIconSources = ({
	name,
	homepage,
	tryNative = false,
}: UsePackageIconSources) => {
	const {
		data: nativeIcon,
		isFetched,
		isFetching,
	} = useQuery({
		...scoopQueries.packageIcon(name, homepage),
		enabled: tryNative && name.length > 0,
	});

	const sources = useMemo(() => {
		if (tryNative && !isFetched) return [];

		const list: string[] = [];
		if (nativeIcon) list.push(nativeIcon);

		// only use remote when native missing or not requested
		if (!tryNative || !nativeIcon) {
			list.push(...getRemoteIconUrls(homepage));
		}

		return list;
	}, [nativeIcon, homepage, tryNative, isFetched]);

	const isResolving = tryNative && isFetching;

	return {
		sources,
		isResolving,
	};
};
