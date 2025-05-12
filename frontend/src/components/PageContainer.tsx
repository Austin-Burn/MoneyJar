export function PageContainer({ children }: PropsWithChildren) {
	return (
		<div
			className={
				"flex h-full w-full flex-col gap-8 rounded-md border border-gray-200 bg-gray-50 p-8 shadow-lg"
			}
		>
			{children}
		</div>
	);
}
