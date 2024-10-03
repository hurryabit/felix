import "@mantine/core/styles.css";
import { ActionIcon, AppShell, Center, Group, MantineProvider, rem, Text } from "@mantine/core";

import { theme, vars } from "./theme";
import Main from "./Main/Main";
import { IconBrandGithubFilled, IconMoonFilled } from "@tabler/icons-react";

export default function App() {
  return (
    <MantineProvider theme={theme}>
      <AppShell
        header={{ height: 64 }}
        footer={{ height: 32 }}
        navbar={{ width: 240, breakpoint: "sm" }}
      >
        <AppShell.Header>
          <Group h="100%" px="md" c="white" bg={vars.colors.blue.filled} justify="space-between">
            <Text>
              <span style={{ fontSize: rem(32), fontWeight: 700, fontFamily: vars.fontFamilyMonospace }}>felix </span>
              Playground for compiler frontend and programming language experiments.
            </Text>
            <Group gap="sm">
              <ActionIcon
                color="white"
                size="md"
                component="a"
                href="https://github.com/hurryabit/felix"
              >
                <IconBrandGithubFilled style={{ width: '70%', height: '70%', color: vars.colors.blue.filled }} stroke={1.5} />
              </ActionIcon>
              <ActionIcon color="white" size="md" onClick={() => alert("Soon!")}>
                <IconMoonFilled style={{ width: '70%', height: '70%', color: vars.colors.blue.filled }} stroke={1.5} />
              </ActionIcon>
            </Group>
          </Group>
        </AppShell.Header>
        <AppShell.Footer>
          <Center  h="100%" w="100%" bg={vars.colors.blue.filled} c="white">
            <Text>
              © 2024 <a href="https://github.com/hurryabit/" style={{color: "white"}}>Martin Huschenbett</a>
            </Text>
          </Center>
        </AppShell.Footer>
        <AppShell.Navbar>
        </AppShell.Navbar>
        <AppShell.Main h="100dvh">
          <Main />
        </AppShell.Main>
      </AppShell>
    </MantineProvider>
  );
}
