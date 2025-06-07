import {useAuthentication} from "@/AuthenticationContext.tsx";
import {MoneyJarRuntime} from "@/runtime.ts";
import {AuthApi} from "@/services/AuthApi.ts";
import {createFileRoute, redirect} from "@tanstack/react-router";
import {Console, Effect, Schema} from "effect";

const FALLBACK = "/" as const;

export const Route = createFileRoute("/login")({
	component: RouteComponent,
	beforeLoad: ({ context, search }) => {
		if (context.auth.isAuthenticated) {
			throw redirect({ to: search.redirect || FALLBACK });
		}
	},
	validateSearch: Schema.standardSchemaV1(
		Schema.Struct({
			redirect: Schema.String,
		}),
	),
});

function RouteComponent() {
	const auth = useAuthentication();

	return (
		<div className="w-full rounded-lg bg-white px-8 py-6 shadow-lg">
			<h1 className="font-bold text-3xl">Login</h1>
			<div className={"w-fit"}>
				<form
					className="mt-4"
					onSubmit={async (e) => {
						e.preventDefault();
						const { email, password } = Object.fromEntries(
							new FormData(e.currentTarget).entries(),
						);

						console.log(email, password);

						AuthApi.login(email as string, password as string).pipe(
							Effect.tap(Console.log),
							Effect.tap((data) => auth.setToken(data.token)),
							MoneyJarRuntime.runPromise,
						);
					}}
				>
					<label className={"mb-2 block"}>
						Email
						<input
							type="email"
							name="email"
							className={
								"mt-1 block w-full rounded-md border border-slate-300 bg-white px-3 py-2 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring focus:ring-indigo-500"
							}
							required
						/>
					</label>
					<label className={"mb-2 block"}>
						Password
						<input
							type="password"
							name="password"
							className={
								"mt-1 block w-full rounded-md border border-slate-300 bg-white px-3 py-2 shadow-sm focus:border-indigo-500 focus:outline-none focus:ring focus:ring-indigo-500"
							}
							required
						/>
					</label>
					<button
						type="submit"
						className={
							"rounded-md bg-indigo-500 px-4 py-2 font-bold text-white hover:bg-indigo-700"
						}
					>
						Login
					</button>
				</form>
			</div>
		</div>
	);
}
