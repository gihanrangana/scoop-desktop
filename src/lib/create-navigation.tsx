import {
	type ComponentType,
	createContext,
	useCallback,
	useContext,
	useMemo,
	useState,
} from "react";
import type {
	Direction,
	NavigateFn,
	NavigationContextValue,
	NavigationOptions,
	ParamsOf,
	RouteUnion,
} from "@/types/navigation";

const createNavigation = <TRoutes extends Record<string, React.FC<never>>>(
	routes: TRoutes,
) => {
	type Params = ParamsOf<TRoutes>;
	type Route = RouteUnion<Params>;
	type CtxValue = NavigationContextValue<Params>;

	const NavigationContext = createContext<CtxValue | null>(null);

	const NavigationProvider: React.FC<{
		children: React.ReactNode;
		initialRoute?: Route;
	}> = ({ children, initialRoute }) => {
		const first = (initialRoute ?? { name: Object.keys(routes)[0] }) as Route;

		const [history, setHistory] = useState<Route[]>([first]);
		const [direction, setDirection] = useState<Direction>("replace");

		const route = history[history.length - 1];

		const navigate = useCallback((...args: [string, ...unknown[]]) => {
			const [name, second, third] = args;

			let params: unknown;
			let options: NavigationOptions | undefined;

			if (second && typeof second === "object" && !("replace" in second)) {
				params = second;
				options = third as NavigationOptions | undefined;
			} else {
				options = second as NavigationOptions | undefined;
			}

			const next = (params ? { name, params } : { name }) as Route;

			setDirection(options?.replace ? "replace" : "forward");
			setHistory((prev) =>
				options?.replace ? [...prev.slice(0, -1), next] : [...prev, next],
			);
		}, []) as NavigateFn<Params>;

		const goBack = useCallback(() => {
			setDirection("back");
			setHistory((prev) => (prev.length > 1 ? prev.slice(0, -1) : prev));
		}, []);

		const isActive = useCallback(
			(...args: [string, ...unknown[]]) => {
				const [name, params] = args;

				if (route.name !== name) return false;

				if (params === undefined) return true;

				return JSON.stringify(route.params) === JSON.stringify(params);
			},
			[route],
		) as CtxValue["isActive"];

		const value = useMemo(
			() => ({
				route,
				direction,
				history,
				canGoBack: history.length > 1,
				isActive,
				navigate,
				goBack,
			}),
			[route, direction, history, navigate, goBack, isActive],
		);

		return (
			<NavigationContext.Provider value={value}>
				{children}
			</NavigationContext.Provider>
		);
	};

	const ActiveScreen: React.FC<{ route: Route }> = ({ route }) => {
		const Component = routes[route.name] as unknown as ComponentType<{
			params?: unknown;
		}>;

		return <Component params={route.params} />;
	};

	const useNavigation = (): CtxValue => {
		const ctx = useContext(NavigationContext);

		if (!ctx)
			throw new Error("useNavigation must be used within NavigationProvider.");

		return ctx;
	};

	return { NavigationProvider, ActiveScreen, useNavigation } as const;
};

export { createNavigation };
