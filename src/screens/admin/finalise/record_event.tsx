import { arrayMoveImmutable } from "array-move";
import React, { RefObject } from "react";
import { SortableContainer, SortableElement } from "react-sortable-hoc";
import HeaderBar from "../../../components/HeaderBar";
import { calculatePointsForRank } from "../../../scoring";
import { store } from "../../../store";
import { EventData, EventType, IndividualData, TeamData } from "../../../types.d";

type RSIProps = {
	event: EventData;
	value: TeamData | IndividualData;
	i: number;
	items: TeamData[] | IndividualData[];
};

const RankingsSortItem = SortableElement<RSIProps>(({ event, value, i, items }: RSIProps) => {
	const [points, setPoints] = React.useState<any>("");

	React.useEffect(() => {
		if (items.length <= 1) {
			setPoints(calculatePointsForRank({ event, position: i + 1 }));
		} else {
			setPoints("...");

			let int = setTimeout(() => {
				setPoints(calculatePointsForRank({ event, position: i + 1 }));
			}, 200);

			return () => clearTimeout(int);
		}
	}, [items]);

	return (
		<div
			className={"row"}
			key={value.id}
			style={
				{
					position: "relative",
					"--col-template": "3rem 1fr 1fr",
					pointerEvents: points == "..." || items.length <= 1 ? "none" : ""
				} as any
			}
		>
			<div className={"col"} style={{ visibility: "hidden" }}>
				{i + 1}
			</div>
			<div className={"col"}>{value.name}</div>
			<div className={"col"}>{points}</div>
		</div>
	);
});

type RSCProps = { event: EventData; items: TeamData[] | IndividualData[] };

const RankingsSortContainer = SortableContainer<RSCProps>(({ event, items }: RSCProps) => {
	return (
		<div className={"body"} style={{ position: "relative" }}>
			{items.map((value, index) => (
				<RankingsSortItem
					key={value.id}
					index={index}
					i={index}
					value={value}
					event={event}
					items={items}
				/>
			))}
		</div>
	);
});

const Rankings = ({
	participants,
	setParticipants,
	event,
	containerRef
}: {
	participants: TeamData[] | IndividualData[];
	setParticipants: any;
	event: EventData;
	containerRef: RefObject<HTMLElement>;
}) => {
	const [sort, setSort] = React.useState(participants);

	React.useEffect(() => {
		if (sort.length == 0) {
			setSort(participants);
		}
	}, [participants]);

	return (
		<RankingsSortContainer
			onSortEnd={({ oldIndex, newIndex }) => {
				let newData = arrayMoveImmutable(sort, oldIndex, newIndex);

				setSort(newData);
				setParticipants(newData);
			}}
			lockAxis={"y"}
			event={event}
			items={sort}
			lockToContainerEdges={true}
			lockOffset={10}
			helperClass={"floating-row"}
		/>
	);
};

export const AdminFinaliseRecordEvent = ({
	id,
	visible,
	event,
	state
}: {
	id: string;
	visible: boolean;
	event: EventData;
	state: { [key: string]: [any, (v: any) => void] };
}) => {
	const [completedTabs, setCompletedTabs] = state.doneTabs;
	const [tabSelected, setTabSelected] = state.selectedTab;

	const [participants, setParticipants] = React.useState<TeamData[] | IndividualData[]>([]);

	const getAllTeams = async () => {
		return store.teams.call<TeamData[]>("get_all_teams").then((res) => {
			setParticipants(res.filter((i) => i.events_ids_entered.includes(event.id)));
		});
	};

	const getAllIndividuals = async () => {
		return store.individuals.call<IndividualData[]>("get_all_individuals").then((res) => {
			setParticipants(res.filter((i) => i.events_ids_entered.includes(event.id)));
		});
	};

	React.useEffect(() => {
		if (visible) {
			if (event.kind == EventType.Individual) {
				getAllIndividuals();
			} else if (event.kind == EventType.Team) {
				getAllTeams();
			}
		}
	}, [visible]);

	const tableRef = React.createRef<HTMLDivElement>();

	return (
		<div className={"teams-create-app"} style={{ display: visible ? "" : "none" }}>
			<HeaderBar
				title={"Results for " + event.name}
				cancel={() => {}}
				cancelProps={{
					className: "btn",
					style: { marginInlineStart: "0.5rem", fontWeight: 600 }
				}}
				cancelText={event.kind == EventType.Individual ? "Individuals" : "Teams"}
				ok={() => {
					if (completedTabs.includes(id)) {
						let newTabs = completedTabs.filter((i: any) => i != id);

						setCompletedTabs(newTabs);
					} else {
						setCompletedTabs([...completedTabs, id]);

						let next = state.tabs[state.tabs.findIndex((t) => t.id == tabSelected) + 1];

						if (!next.id.startsWith("ev-")) {
							next = state.tabs[state.tabs.findIndex((t) => t.id == tabSelected) + 2];
						}

						setTabSelected(next.id);
					}
				}}
				okText={completedTabs.includes(id) ? "Unmark as done" : "Mark event as done"}
				okProps={{
					className: `btn ${completedTabs.includes(id) ? `secondary` : `primary`} small`
				}}
			/>

			<div className="intro-container record-container">
				<div className={"intro-whoami record-table-container"}>
					<div
						className={"table-grid is-record-table"}
						ref={tableRef}
						style={{ "--col-template": "3rem 1fr 1fr", position: "relative" } as any}
					>
						<div className={"head"}>
							<div className={"col"}>#</div>
							<div className={"col"}>
								{event.kind == EventType.Individual ? "Individual" : "Team Name"}
							</div>
							<div className={"col"}>Calculated Points</div>
						</div>

						<div
							className={"body fixed-id"}
							style={
								{
									position: "absolute",
									top: "3rem",
									zIndex: 0,
									width: "100%"
								} as any
							}
						>
							{[...Array(participants.length)].map((_, index) => (
								<div className={"row"}>
									<div className="col">{index + 1}</div>
									<div className="col"></div>
									<div className="col"></div>
								</div>
							))}
						</div>

						<Rankings
							participants={participants}
							setParticipants={setParticipants}
							event={event}
							containerRef={tableRef}
						/>
					</div>
				</div>

				<p
					style={{
						textAlign: "center",
						margin: "3rem auto",
						marginBottom: 0,
						maxWidth: "380px"
					}}
				>
					Rearrange {event.kind == EventType.Individual ? "the individuals" : "teams"}{" "}
					into correct ranking order, and mark as done when complete.
				</p>
			</div>
		</div>
	);
};
