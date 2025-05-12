import { PageContainer } from "@/components/PageContainer.tsx";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/groups")({
	component: RouteComponent,
});

function RouteComponent() {
	return (
		<PageContainer>
			<div className={"flex flex-row items-center justify-between"}>
				<h1 className={"text-4xl"}>Groups</h1>
			</div>
		</PageContainer>
	);
}
