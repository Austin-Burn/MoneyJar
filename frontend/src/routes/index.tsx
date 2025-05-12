import { PageContainer } from "@/components/PageContainer.tsx";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/")({
	component: App,
});

function App() {
	return (
		<PageContainer>
			<div className={"flex flex-row items-center justify-between"}>
				<h1 className={"text-4xl"}>Index</h1>
			</div>
		</PageContainer>
	);
}
