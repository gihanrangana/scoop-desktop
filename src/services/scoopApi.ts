import { invoke } from "@tauri-apps/api/core";
import type {
	Bucket,
	InstalledPackage,
	Package,
	PackageDetails,
	ScoopStats,
} from "@/types/scoop";

export const getAllPackages = async (): Promise<Package[]> =>
	await invoke<Package[]>("get_all_packages");

export const getPackagesByBucket = async (
	bucketName: string,
): Promise<Package[]> =>
	await invoke<Package[]>("get_packages_by_bucket", { bucketName });

export const searchPackages = async (query: string): Promise<Package[]> =>
	await invoke<Package[]>("search_packages", { query });

export const getInstalledPackages = async (): Promise<InstalledPackage[]> =>
	await invoke<InstalledPackage[]>("get_installed_packages");

export const getBuckets = async (): Promise<Bucket[]> =>
	await invoke<Bucket[]>("get_buckets");

export const checkIfInstalled = async (name: string): Promise<boolean> =>
	await invoke<boolean>("check_if_installed", { name });

export const installPackage = async (name: string): Promise<string> =>
	await invoke<string>("install_package", { name });

export const uninstallPackage = async (name: string): Promise<string> =>
	await invoke<string>("uninstall_package", { name });

export const getScoopStats = async (): Promise<ScoopStats> => {
	const [installed, buckets] = await Promise.all([
		getInstalledPackages(),
		getBuckets(),
	]);

	return {
		installed: installed.length,
		buckets: buckets.length,
		packages: buckets.reduce((acc, bucket) => acc + bucket.package_count, 0),
	};
};

export const getRecentInstalledPackages = async (
	limit?: number,
): Promise<InstalledPackage[]> =>
	await invoke<InstalledPackage[]>("get_recent_installed_packages", { limit });

export const getPackageInfo = async (
	name: string,
	bucket: string,
): Promise<PackageDetails> => {
	return await invoke<PackageDetails>("get_package_info", { name, bucket });
};

export const getPackageIcon = async (
	name: string,
	homepage?: string,
): Promise<string | null> =>
	await invoke<string | null>("get_package_icon", {
		pkgName: name,
		homepage: homepage ?? null,
	});
