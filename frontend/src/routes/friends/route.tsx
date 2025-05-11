import {
	mdiAccountPlus,
	mdiCashFast,
	mdiDotsVertical,
	mdiMagnify,
	mdiQrcode,
} from "@mdi/js";
import Icon from "@mdi/react";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/friends")({
	component: RouteComponent,
});

const Users = [
	{ name: "John Smith", id: 0 },
	{ name: "John Smith", id: 1 },
	{ name: "John Smith", id: 2 },
	{ name: "John Smith", id: 3 },
	{ name: "John Smith", id: 4 },
	{ name: "John Smith", id: 5 },
	{ name: "John Smith", id: 6 },
	{ name: "John Smith", id: 7 },
	{ name: "John Smith", id: 8 },
	{ name: "John Smith", id: 9 },
	{ name: "John Smith", id: 10 },
	{ name: "John Smith", id: 11 },
	{ name: "John Smith", id: 12 },
	{ name: "John Smith", id: 13 },
	{ name: "John Smith", id: 14 },
	{ name: "John Smith", id: 15 },
	{ name: "John Smith", id: 16 },
	{ name: "John Smith", id: 17 },
	{ name: "John Smith", id: 18 },
	{ name: "John Smith", id: 19 },
	{ name: "John Smith", id: 20 },
	{ name: "John Smith", id: 21 },
	{ name: "John Smith", id: 22 },
	{ name: "John Smith", id: 23 },
	{ name: "John Smith", id: 24 },
	{ name: "John Smith", id: 25 },
	{ name: "John Smith", id: 26 },
	{ name: "John Smith", id: 27 },
	{ name: "John Smith", id: 28 },
	{ name: "John Smith", id: 29 },
	{ name: "John Smith", id: 30 },
];

function RouteComponent() {
	return (
		<div
			className={
				"flex h-full w-full flex-col gap-8 rounded-md border border-gray-200 bg-gray-50 p-4 shadow-lg"
			}
		>
			<div className={"flex flex-row items-center justify-between"}>
				<h1 className={"text-4xl"}>Friends</h1>
				<div className={"flex flex-row items-center gap-4"}>
					<Icon className={"text-gray-700"} path={mdiQrcode} size={1.0} />
					<Icon className={"text-gray-700"} path={mdiAccountPlus} size={1.0} />
					<div
						className={
							"flex h-fit w-48 cursor-text justify-end rounded-lg border-2 border-gray-300 bg-white px-2 py-0.5 shadow-sm"
						}
					>
						<Icon className={"text-gray-700"} path={mdiMagnify} size={1.0} />
					</div>
				</div>
			</div>

			<div
				className={
					"flex h-fit max-h-full w-full flex-col overflow-y-auto border-gray-300 border-y"
				}
			>
				{Users.map((user) => (
					<div
						key={user.id}
						className={
							"flex flex-row justify-between border-gray-300 not-last:border-b px-2 py-4 hover:bg-gray-100"
						}
					>
						<p>{user.name}</p>
						<div className={"flex flex-row items-center gap-12"}>
							<div className={"flex flex-row items-center gap-4"}>
								<p className={"text-red-700"}>-$10</p>
								<Icon path={mdiCashFast} size={1.0} />
							</div>
							<Icon path={mdiDotsVertical} size={1.0} />
						</div>
					</div>
				))}
			</div>
		</div>
	);
}
