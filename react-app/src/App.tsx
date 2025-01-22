import { BrowserRouter, Routes } from "react-router";
import { Route } from "react-router";
import { AccessPage } from "./components/access-page";

type ObjectValue<T> = T[keyof T];

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<AccessPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
