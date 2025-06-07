// AuthenticationContext
import {createContext, type PropsWithChildren, useContext, useState,} from "react";

export const authenticationContext = createContext<{
	isAuthenticated: boolean;
	token: string | undefined;
	setToken: (token: string | undefined) => void;
}>({
	isAuthenticated: false,
	token: undefined,
	setToken: () => {},
});

export const AuthenticationProvider = ({ children }: PropsWithChildren) => {
	const [token, setToken] = useState<string | undefined>(undefined);
	const isAuthenticated = token !== undefined;

	return (
		<authenticationContext.Provider
			value={{ isAuthenticated, token, setToken }}
		>
			{children}
		</authenticationContext.Provider>
	);
};

export function useAuthentication() {
	const context = useContext(authenticationContext);
	if (context === undefined) {
		throw new Error(
			"useAuthentication must be used within a AuthenticationProvider",
		);
	}
	return context;
}
