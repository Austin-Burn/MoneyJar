import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/account")({
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<div
			className={
				"flex h-full w-full flex-col gap-8 rounded-md border border-gray-200 bg-gray-50 p-4 shadow-lg"
			}
		>
			<div className={"flex flex-row items-center justify-between"}>
				<h1 className={"text-4xl"}>Account</h1>
			</div>
		</div>
	);
}
