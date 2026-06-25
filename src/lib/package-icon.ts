import { parseUrl } from "@/lib/utils";

export const getClearbitLogoUrl = (
	homepage: string | null | undefined,
): string | null => {
	const url = homepage ? parseUrl(homepage) : null;
	if (!url?.hostname) return null;
	return `https://logo.clearbit.com/${url.hostname}`;
};
export const getGoogleFaviconUrl = (
	homepage: string | null | undefined,
): string | null => {
	const url = homepage ? parseUrl(homepage) : null;
	if (!url?.hostname) return null;
	return `https://www.google.com/s2/favicons?domain=${url.hostname}&sz=64`;
};
export const getGithubAvatarUrl = (
	homepage: string | null | undefined,
): string | null => {
	if (!homepage) return null;
	try {
		const url = new URL(homepage);
		if (!url.hostname.includes("github.com")) return null;
		const [, owner] = url.pathname.split("/").filter(Boolean);
		return owner ? `https://github.com/${owner}.png?size=64` : null;
	} catch {
		return null;
	}
};
export const getLetterFallback = (name: string): string => {
	return name.charAt(0).toUpperCase() || "?";
};

export const getRemoteIconUrls = (
	homepage: string | null | undefined,
): string[] => {
	return [
		getClearbitLogoUrl(homepage), // 404 for openssl.org → onError
		getGoogleFaviconUrl(homepage), // backup
		getGithubAvatarUrl(homepage),
	].filter((url): url is string => Boolean(url));
};
