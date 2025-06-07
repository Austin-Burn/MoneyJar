import {PageContainer} from "@/components/PageContainer.tsx";
import {MoneyJarRuntime} from "@/runtime.ts";
import {EventsApi} from "@/services/EventsApi.ts";
import {WhoInWhatApi} from "@/services/WhoInWhatApi.ts";
import {useSuspenseQuery} from "@tanstack/react-query";
import {createFileRoute} from "@tanstack/react-router";
import {Console, Effect} from "effect";

export const Route = createFileRoute("/events")({
	component: RouteComponent,
});

function RouteComponent() {
	const { data } = useSuspenseQuery({
		queryKey: ["events"],
		queryFn: () =>
			WhoInWhatApi.getEventsFromUser(
				"60513378-ae64-48f7-95c5-42fd36046573",
			).pipe(
				Effect.tap((data) => Console.log(data)),
				Effect.flatMap((data) =>
					Effect.all(data.events.map((id) => EventsApi.getEvent(id))),
				),
				Effect.tap((data) => Console.log(data)),
				MoneyJarRuntime.runPromise,
			),
	});

	return (
		<PageContainer>
			<div className={"flex flex-row items-center justify-between"}>
				<h1 className={"text-4xl"}>Events</h1>
			</div>
		</PageContainer>
	);
}
