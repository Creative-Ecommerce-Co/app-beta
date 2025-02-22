// project imports
import Dashboard from "./dashboard/default";

// export meta
export const meta = () => ({
  title: "Dashboard | Berry - React Material Admin Dashboard Template",
  description:
    "Start your next React project with Berry admin template. It build with Reactjs, Material-UI, Redux, and Hook for faster web development.",
});

// ==============================|| DEFAULT PAGE ||============================== //

const Index: React.FC = () => {
  return (
    <>
      <Dashboard />
    </>
  );
}

export default Index;