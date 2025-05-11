import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
	component: App,
});

function App() {
	return <div className={"h-full w-full grow bg-amber-100"}>index</div>;
}
