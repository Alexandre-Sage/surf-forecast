import {
  ChakraProvider,
  createSystem,
  defaultSystem,
  defineConfig,
  Theme,
} from "@chakra-ui/react";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import { ColorModeProvider } from "./components/ui";
import "./i18n";
import "normalize.css";
import {
  QueryClient,
  QueryClientProvider,
  useQueryClient,
} from "@tanstack/react-query";
const config = defineConfig({
  theme: {
    tokens: {
      colors: {},
    },
  },
});

const system = createSystem(config);
const queryClient = new QueryClient();
createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <ChakraProvider value={defaultSystem}>
      <ColorModeProvider defaultTheme="dark">
        <Theme colorPalette="teal">
          <QueryClientProvider client={queryClient}>
            <App />
          </QueryClientProvider>
        </Theme>
      </ColorModeProvider>
    </ChakraProvider>
  </StrictMode>,
);
