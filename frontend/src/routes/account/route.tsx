import {PageContainer} from "@/components/PageContainer.tsx";
import {MoneyJarRuntime} from "@/runtime.ts";
import {UserApi} from "@/services/UserApi.ts";
import {formatPhoneNumber} from "@/util.ts";
import {mdiImageEdit, mdiQrcode, mdiTrashCan} from "@mdi/js";
import Icon from "@mdi/react";
import {useSuspenseQuery} from "@tanstack/react-query";
import {createFileRoute} from "@tanstack/react-router";
import type {PropsWithChildren} from "react";
import AccountDefaultSvg from "../../images/account-default.svg?react";

export const Route = createFileRoute("/account")({
	component: RouteComponent,
});

function RouteComponent() {
	const { data } = useSuspenseQuery({
		queryKey: ["user"],
		queryFn: () =>
			UserApi.getUserInfo("60513378-ae64-48f7-95c5-42fd36046573").pipe(
				MoneyJarRuntime.runPromise,
			),
	});

	return (
		<PageContainer>
			<div className={"flex flex-row items-center justify-between"}>
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
						<h1 className={"font-semibold text-3xl"}>{data.name}</h1>
						<p className={"text-gray-400"}>@{data.id}</p>
					</div>
				</div>
				<div className={"flex flex-row items-center gap-4"}>
					<Icon className={"text-gray-700"} path={mdiQrcode} size={1.0} />
				</div>
			</div>
			<div className={"flex flex-col gap-4"}>
				<Info label={"Email"}>{data.email}</Info>
				<Info label={"Phone"}>{formatPhoneNumber(data.phone)}</Info>
				<Info label={"Password"}>●●●●●●●●●●●●</Info>
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
		</PageContainer>
	);
}

type InfoProps = {
	label: string;
};

function Info({ label, children }: PropsWithChildren<InfoProps>) {
	return (
		<div>
			<h2 className={"font-semibold text-2xl"}>{label}</h2>
			<p className={"text-lg"}>{children}</p>
		</div>
	);
}
