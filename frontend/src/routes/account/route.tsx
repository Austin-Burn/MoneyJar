import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/account")({
	component: RouteComponent,
});

function RouteComponent() {
	return <div className={"h-full w-full grow bg-amber-100"}>account</div>;
}
