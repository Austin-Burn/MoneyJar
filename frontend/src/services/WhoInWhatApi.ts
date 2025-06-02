import {GetEventsFromUserRequest} from "@/schema/api/whoInWhat.ts";
import {createApiRequestFunction} from "@/util.ts";
import {HttpClient} from "@effect/platform";
import {Effect} from "effect";

export class WhoInWhatApi extends Effect.Service<WhoInWhatApi>()(
	"WhoInWhatApi",
	{
		accessors: true,
		effect: Effect.gen(function* () {
			yield* Effect.logTrace("Setting up WhoInWhatApi");

			const httpClient = yield* HttpClient.HttpClient.pipe(
				Effect.map(HttpClient.filterStatusOk),
			);

			const apiRequest = createApiRequestFunction(httpClient);

			return {
				getEventsFromUser: (userId: string) =>
					apiRequest(
						"/GetEventsFromUser",
						{ user_id: userId },
						GetEventsFromUserRequest,
					),
			};
		}),
	},
) {}
