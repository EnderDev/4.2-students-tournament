import React from "react";
import { useNavigate } from "react-router-dom";
import HeaderBar from "../../components/HeaderBar";
import Sidebar from "../../components/Sidebar";
import { AdminDashboard } from "./dashboard";
import { AdminEvents } from "./events";
import { AdminIndividuals } from "./individuals";
import { AdminTeams } from "./teams";

// Register all the tabs in the admin mode.
const tabs = [
	{
		id: "dashboard",
		name: "Dashboard",
		render: () => <AdminDashboard />
	},
	{
		id: "events",
		name: "Events",
		render: () => <AdminEvents />
	},
	{
		id: "individuals",
		name: "Individuals",
		render: () => <AdminIndividuals />
	},
	{
		id: "teams",
		name: "Teams",
		render: () => <AdminTeams />
	}
];

// Wraps all the other admin screens in a sidebar.
const AdminMain = () => {
	const navigate = useNavigate();

	// Store the current tab in the state.
	const [currentTab, setCurrentTab] = React.useState<string>(tabs[0].id);

	return (
		<>
			<Sidebar
				title={"Admin Mode"}
				goBack={() => navigate("/")}
				tabs={tabs}
				state={[currentTab, setCurrentTab]}
			>
				<HeaderBar />
			</Sidebar>
		</>
	);
};

export default AdminMain;
