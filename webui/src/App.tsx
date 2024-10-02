import "@mantine/core/styles.css";
import { AppShell, MantineProvider } from "@mantine/core";

import { theme } from "./theme";
import Main from "./Main/Main";

export default function App() {
  return (
    <MantineProvider theme={theme}>
      <AppShell
        header={{ height: 60 }}
        navbar={{ width: 240, breakpoint: "sm" }}
      >
        <AppShell.Header></AppShell.Header>
        <AppShell.Navbar>
        </AppShell.Navbar>
        <AppShell.Main h="100dvh">
          <Main />
        </AppShell.Main>
      </AppShell>
    </MantineProvider>
  );
}
