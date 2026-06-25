import {
	type QueriesOptions,
	type QueriesResults,
	useQueries,
} from "@tanstack/react-query";

export const useScoop = <T extends Array<unknown>>(
	queries: readonly [...QueriesOptions<T>],
): QueriesResults<T> => {
	return useQueries({ queries });
};
