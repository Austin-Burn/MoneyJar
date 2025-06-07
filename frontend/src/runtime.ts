import {AuthApi} from "@/services/AuthApi.ts";
import {EventsApi} from "@/services/EventsApi.ts";
import {UserApi} from "@/services/UserApi.ts";
import {WhoInWhatApi} from "@/services/WhoInWhatApi.ts";
import {FetchHttpClient} from "@effect/platform";
import {Layer, ManagedRuntime} from "effect";

const MainLayer = Layer.mergeAll(
	UserApi.Default,
	WhoInWhatApi.Default,
	EventsApi.Default,
	AuthApi.Default,
).pipe(Layer.provide(FetchHttpClient.layer));

export const MoneyJarRuntime = ManagedRuntime.make(MainLayer);
