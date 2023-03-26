export const AdminDashboard = (
	rest: React.DetailedHTMLProps<React.HTMLAttributes<HTMLDivElement>, HTMLDivElement>
) => {
	return (
		<div {...rest} className="center-container admin-container">
			<div className={"admin-main center-container"}>
				<h1>Dashboard</h1>
				<p>Internal tournament settings</p>
			</div>
		</div>
	);
};
