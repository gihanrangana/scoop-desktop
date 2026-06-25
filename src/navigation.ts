import { createNavigation } from "@/lib/create-navigation";
import BrowseScreen from "@/screens/BrowseScreen";
import BucketsScreen from "@/screens/BucketsScreen";
import HomeScreen from "@/screens/HomeScreen";
import InstalledScreen from "@/screens/InstalledScreen";
import type { NoParamKeys, ParamsOf, RouteUnion } from "@/types/navigation";

const routes = {
	home: HomeScreen,
	browse: BrowseScreen,
	installed: InstalledScreen,
	buckets: BucketsScreen,
};

type AppParams = ParamsOf<typeof routes>;

/** All route names */
export type AppRouteName = RouteUnion<AppParams>["name"];

/** Routes with no params — good for sidebar links */
export type AppNavRouteName = NoParamKeys<AppParams>;

export const { NavigationProvider, ActiveScreen, useNavigation } =
	createNavigation(routes);
