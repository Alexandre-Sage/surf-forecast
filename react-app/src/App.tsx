import { BrowserRouter, Routes } from "react-router";
import { Route } from "react-router";
import { AccessPage } from "./components/access-page";
import { UserPage } from "./components/user";

type ObjectValue<T> = T[keyof T];

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<AccessPage />} />
        <Route path="/user/:id" element={<UserPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
