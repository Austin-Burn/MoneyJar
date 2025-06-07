import {createApiRequestFunction} from "@/util.ts";
import {HttpClient} from "@effect/platform";
import {Effect, Schema} from "effect";

export class AuthApi extends Effect.Service<AuthApi>()("AuthApi", {
	accessors: true,
	effect: Effect.gen(function* () {
		yield* Effect.logTrace("Setting up AuthApi");

		const httpClient = yield* HttpClient.HttpClient.pipe(
			Effect.map(HttpClient.filterStatusOk),
		);

		const apiRequest = createApiRequestFunction(httpClient);

		return {
			login: (email: string, password: string) =>
				apiRequest(
					"/Login",
					{ email, password },
					Schema.Struct({
						id: Schema.UUID,
						token: Schema.String,
					}),
				),
		};
	}),
}) {}
