import {UserApi} from "@/services/UserApi.ts";
import {FetchHttpClient} from "@effect/platform";
import {Layer, ManagedRuntime} from "effect";

const MainLayer = Layer.mergeAll(UserApi.Default).pipe(
	Layer.provide(FetchHttpClient.layer),
);

export const MoneyJarRuntime = ManagedRuntime.make(MainLayer);
