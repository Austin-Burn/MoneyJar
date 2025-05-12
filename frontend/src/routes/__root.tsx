import {
	mdiAccount,
	mdiAccountGroup,
	mdiCalendar,
	mdiCircle,
	mdiHome,
} from "@mdi/js";
import Icon from "@mdi/react";
import { Link, Outlet, createRootRoute } from "@tanstack/react-router";
import AccountDefaultSvg from "../images/account-default.svg?react";

export const Route = createRootRoute({
	component: () => (
		<div className={"flex h-full w-full justify-center"}>
			{/*	<TanStackRouterDevtools />*/}
			<div className={"relative flex h-full w-2/3 flex-row py-6"}>
				<div className={"-translate-x-full absolute left-0"}>
					<nav className={"flex flex-col justify-start gap-1 p-4"}>
						{/* Home page */}
						<Link
							to={"/"}
							className={
								"flex h-12 w-12 cursor-pointer items-center justify-center rounded-full hover:bg-gray-200 active:bg-gray-400"
							}
						>
							<Icon className={"text-gray-700"} path={mdiHome} size={1.0} />
						</Link>

						{/* Events page */}
						<Link
							to={"/events"}
							className={
								"flex h-12 w-12 cursor-pointer items-center justify-center rounded-full hover:bg-gray-200 active:bg-gray-400"
							}
						>
							<Icon className={"text-gray-700"} path={mdiCalendar} size={1.0} />
						</Link>

						{/* Friends page */}
						<Link
							to={"/friends"}
							className={
								"flex h-12 w-12 cursor-pointer items-center justify-center rounded-full hover:bg-gray-200 active:bg-gray-400"
							}
						>
							<Icon className={"text-gray-700"} path={mdiAccount} size={1.0} />
						</Link>

						{/* Groups page */}
						<Link
							to={"/groups"}
							className={
								"flex h-12 w-12 cursor-pointer items-center justify-center rounded-full hover:bg-gray-200 active:bg-gray-400"
							}
						>
							<Icon
								className={"text-gray-700"}
								path={mdiAccountGroup}
								size={1.0}
							/>
						</Link>

						{/* Account page */}
						<Link
							to={"/account"}
							className={
								"flex h-12 w-12 cursor-pointer items-center justify-center rounded-full hover:bg-gray-200 active:bg-gray-400"
							}
						>
							<div className={"h-10 w-10"}>
								<AccountDefaultSvg />
							</div>
						</Link>
					</nav>
				</div>
				<Outlet />
			</div>
		</div>
	),
});
