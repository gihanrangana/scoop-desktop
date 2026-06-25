import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export const parseUrl = (url: string) => {
	const trimmed = url.trim();
	if (!trimmed) return null;

	try {
		if (trimmed.startsWith("//")) return new URL(`https:${trimmed}`);

		if (/^https?:\/\//i.test(trimmed)) return new URL(trimmed);

		return new URL(`https://${trimmed}`);
	} catch {
		return null;
	}
};
