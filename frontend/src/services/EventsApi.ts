import {GetEventRequest} from "@/schema/api/event.ts";
import {createApiRequestFunction} from "@/util.ts";
import {HttpClient} from "@effect/platform";
import {Effect} from "effect";

export class EventsApi extends Effect.Service<EventsApi>()("EventsApi", {
	accessors: true,
	effect: Effect.gen(function* () {
		yield* Effect.logTrace("Setting up EventsApi");

		const httpClient = yield* HttpClient.HttpClient.pipe(
			Effect.map(HttpClient.filterStatusOk),
		);

		const apiRequest = createApiRequestFunction(httpClient);

		return {
			getEvent: (id: string) =>
				apiRequest("/GetEvent", { id }, GetEventRequest),
		};
	}),
}) {}
