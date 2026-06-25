"use client";

import type Lenis from "lenis";
import { createContext, useContext } from "react";

const LenisContext = createContext<Lenis | null>(null);

export const LenisProvider = LenisContext.Provider;

export const useLenis = () => {
	return useContext(LenisContext);
};
