export interface Package {
	name: string;
	version: string;
	description: string;
	homepage: string;
	notes: string[];
	bucket: string;
	license?: string | null;
}

export interface InstalledPackage {
	name: string;
	version: string;
	bucket: string;
	install_path: string;
	installed_at: number;
	homepage: string | null;
	description: string;
}

export interface Bucket {
	name: string;
	package_count: number;
}

export type ScoopStats = {
	installed: number;
	buckets: number;
	packages: number;
};

export type UseScoopOptions = {
	stats?: boolean;
	installed?: boolean;
	buckets?: boolean;
	packages?: boolean;
};

export type UseScoopResult = {
	stats: ScoopStats | null;
	installed: InstalledPackage[] | null;
	buckets: Bucket[] | null;
	packages: Package[] | null;
	isLoading: boolean;
	error: string | null;
	refresh: () => Promise<void>;
};

export type PackageDetails = {
	version: string;
	description: string[] | null;
	homepage: string;
	notes: string[];
	bucket: string;
	license: string | null;
	architecture: Record<string, unknown> | null;
	depends: string[] | null;
	bin: string | string[] | Record<string, unknown> | null;
};
