import { Outlet } from "react-router-dom";
import NavigationBar from "@/components/NavigationBar";

function RootLayout() {
  return (
    <div>
      <NavigationBar />
      <main>
        <Outlet />
      </main>
    </div>
  );
}

export default RootLayout;
