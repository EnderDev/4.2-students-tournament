import autoAnimate from "@formkit/auto-animate";
import React, { Dispatch, SetStateAction } from "react";
import HeaderBar from "./HeaderBar";

// Tab interface for the sidebar
export interface Tab {
	id: string;
	name: string | (() => any);
	render: () => any;
	disabled?: boolean;
}

// Used to render the tabs in the sidebar
const Tabs = ({
	list,
	classNames,
	visible,
	setVisible,
	animated
}: {
	list: Tab[];
	classNames?: any;
	visible: string;
	setVisible: any;
	animated: boolean;
}) => {
	const parentRef = React.createRef<HTMLDivElement>();

	React.useEffect(() => {
		if (parentRef.current && animated) {
			autoAnimate(parentRef.current);
		}
	}, [parentRef]);

	return (
		<div className={["sidebar-items", classNames?.items || ""].join(" ")} ref={parentRef}>
			{list.map((t, index) => {
				return (
					<button
						key={t.id}
						className={"btn secondary"}
						data-selected={visible == t.id}
						onClick={() => setVisible(t.id)}
						disabled={t.disabled}
					>
						{typeof t.name == "function" ? <t.name /> : t.name}
					</button>
				);
			})}
		</div>
	);
};

// Sidebar component
const Sidebar = ({
	title,
	goBack,
	goBackProps,
	goForward,
	goForwardProps,
	exit,
	exitProps,
	content,
	state,
	tabs,
	children,
	classNames,
	animatedTabs
}: {
	title?: string | (() => any);
	goBack?: (...rest: any) => any;
	goBackProps?: any;
	goForward?: (...rest: any) => any;
	goForwardProps?: any;
	exit?: (...rest: any) => any;
	exitProps?: any;
	content?: any;
	state: [string, Dispatch<SetStateAction<string>>];
	tabs: Tab[];
	children?: any;
	classNames?: Record<string, string>;
	animatedTabs?: boolean;
}) => {
	let [visible, setVisible] = state;

	return (
		<aside className={["sidebar", classNames?.root || ""].join(" ")}>
			<div className={["sidebar-container", classNames?.container || ""].join(" ")}>
				<HeaderBar
					title={title}
					goBack={goBack}
					goBackProps={goBackProps}
					goForward={goForward}
					goForwardProps={goForwardProps}
					exit={exit}
					exitProps={exitProps}
				/>

				<div className={["sidebar-items", classNames?.items || ""].join(" ")}>
					<Tabs
						list={tabs}
						visible={visible}
						setVisible={setVisible}
						animated={!!animatedTabs}
					/>
				</div>
			</div>

			<main className={"sidebar-content"}>
				{children}

				{tabs.map((t) => (
					<>
						{React.cloneElement(t.render() || <></>, {
							key: t.id,
							"data-visible": visible == t.id
						})}
					</>
				))}
			</main>
		</aside>
	);
};

export default Sidebar;
