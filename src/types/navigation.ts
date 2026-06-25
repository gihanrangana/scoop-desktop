import type React from "react";

export type InferParams<C> =
	C extends React.FC<infer P>
		? P extends { params: infer Params }
			? Params
			: undefined
		: undefined;

export type ParamsOf<T extends Record<string, React.FC<never>>> = {
	[K in keyof T & string]: InferParams<T[K]>;
};

export type NoParamKeys<P> = {
	[K in keyof P & string]: P[K] extends undefined ? K : never;
}[keyof P & string];

export type WithParamKeys<P> = Exclude<keyof P & string, NoParamKeys<P>>;

export type RouteUnion<P> = {
	[K in keyof P & string]: P[K] extends undefined
		? { name: K; params?: undefined }
		: { name: K; params: P[K] };
}[keyof P & string];

export type NavigationOptions = { replace?: boolean };

export type NavigateFn<P> = (<T extends NoParamKeys<P>>(
	name: T,
	options?: NavigationOptions,
) => void) &
	(<T extends WithParamKeys<P>>(
		name: T,
		params: P[T],
		options?: NavigationOptions,
	) => void);

export type IsActiveFN<P> = <T extends NoParamKeys<P>>(
	name: T,
) => boolean & (<T extends WithParamKeys<P>>(name: T, params: P[T]) => boolean);

export type Direction = "forward" | "back" | "replace";

export type NavigationContextValue<P> = {
	route: RouteUnion<P>;
	direction: Direction;
	history: RouteUnion<P>[];
	canGoBack: boolean;
	isActive: IsActiveFN<P>;
	navigate: NavigateFn<P>;
	goBack: () => void;
};
