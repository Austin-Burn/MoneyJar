import { mdiImageEdit, mdiQrcode, mdiTrashCan } from "@mdi/js";
import Icon from "@mdi/react";
import { createFileRoute } from "@tanstack/react-router";
import AccountDefaultSvg from "../../images/account-default.svg?react";

export const Route = createFileRoute("/account")({
	component: RouteComponent,
});

const User = {
	id: "1358eb04-b790-4793-a4bc-67a48171b02f",
	name: "John Smith",
	email: "john.smith@example.com",
};

function RouteComponent() {
	return (
		<div
			className={
				"flex h-full w-full flex-col gap-4 rounded-md border border-gray-200 bg-gray-50 p-4 shadow-lg"
			}
		>
			<div className={"flex flex-row items-center justify-between p-4"}>
				<div className={"flex flex-row items-center gap-8"}>
					<div
						className={
							"relative h-36 w-36 overflow-hidden rounded-full shadow-md hover:bg-blend-darken"
						}
					>
						<div
							className={
								"absolute top-0 z-10 flex h-36 w-36 cursor-pointer items-center justify-center bg-[rgba(0,0,0,0.25)] opacity-0 hover:opacity-100"
							}
						>
							<Icon className={"text-white"} path={mdiImageEdit} size={1.0} />
						</div>
						<AccountDefaultSvg />
					</div>
					<div className={"flex flex-col"}>
						<h1 className={"font-semibold text-3xl"}>{User.name}</h1>
						<p className={"text-gray-400"}>@{User.id}</p>
					</div>
				</div>
				<div className={"flex flex-row items-center gap-4"}>
					<Icon className={"text-gray-700"} path={mdiQrcode} size={1.0} />
				</div>
			</div>
			<div className={"flex flex-col gap-4 px-4 "}>
				<div>
					<h2 className={"font-semibold text-2xl"}>Email</h2>
					<p className={"text-lg"}>{User.email}</p>
				</div>
				<div>
					<h2 className={"font-semibold text-2xl"}>Password</h2>
					<p className={"text-lg"}>●●●●●●●●●●●●</p>
				</div>
				<div className={"flex flex-row rounded-md border border-red-400 p-4"}>
					<button
						className={
							"flex cursor-pointer flex-row gap-2 rounded-md bg-red-400 px-4 py-2 text-white hover:bg-red-500 active:bg-red-300"
						}
						type={"button"}
					>
						<p>Delete Account</p>
						<Icon path={mdiTrashCan} size={1.0} />
					</button>
				</div>
			</div>
		</div>
	);
}
