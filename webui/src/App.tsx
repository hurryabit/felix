import "@mantine/core/styles.css";

import { useEffect } from "react";
import {
    IconBrandGithubFilled,
    IconMaximize,
    IconMinimize,
    IconMoonFilled,
} from "@tabler/icons-react";
import * as wasm from "felix-wasm-bridge";
import { ActionIcon, AppShell, Center, Group, MantineProvider, rem, Text } from "@mantine/core";
import { useFullscreen } from "@mantine/hooks";
import AppStateProvider from "./AppStateProvider";
import Main from "./Main/Main";
import { theme, vars } from "./theme";

export default function App() {
    const { fullscreen, toggle: toggleFullscreen } = useFullscreen();

    useEffect(function () {
        console.log(`One day, we'll type check using the ${wasm.type_system_name()}.`);
    }, []);

    return (
        <MantineProvider theme={theme}>
            <AppStateProvider>
                <AppShell
                    header={{ height: 64 }}
                    footer={{ height: 32 }}
                    navbar={{ width: 240, breakpoint: "sm" }}
                >
                    <AppShell.Header>
                        <Group
                            h="100%"
                            px="md"
                            c="white"
                            bg={vars.colors.blue.filled}
                            justify="space-between"
                        >
                            <Text>
                                <span
                                    style={{
                                        fontSize: rem(32),
                                        fontWeight: 700,
                                        fontFamily: vars.fontFamilyMonospace,
                                    }}
                                >
                                    feλix{" "}
                                </span>
                                Playground for compiler frontend and programming language
                                experiments.
                            </Text>
                            <Group gap="sm">
                                <ActionIcon
                                    color="white"
                                    size="md"
                                    component="a"
                                    href="https://github.com/hurryabit/felix"
                                >
                                    <IconBrandGithubFilled
                                        style={{
                                            width: "70%",
                                            height: "70%",
                                            color: vars.colors.blue.filled,
                                        }}
                                        stroke={1.5}
                                    />
                                </ActionIcon>
                                <ActionIcon color="white" size="md" onClick={() => alert("Soon!")}>
                                    <IconMoonFilled
                                        style={{
                                            width: "70%",
                                            height: "70%",
                                            color: vars.colors.blue.filled,
                                        }}
                                        stroke={1.5}
                                    />
                                </ActionIcon>
                                <ActionIcon color="white" size="md" onClick={toggleFullscreen}>
                                    {fullscreen ? (
                                        <IconMinimize
                                            style={{
                                                width: "70%",
                                                height: "70%",
                                                color: vars.colors.blue.filled,
                                            }}
                                            stroke={1.5}
                                        />
                                    ) : (
                                        <IconMaximize
                                            style={{
                                                width: "70%",
                                                height: "70%",
                                                color: vars.colors.blue.filled,
                                            }}
                                            stroke={1.5}
                                        />
                                    )}
                                </ActionIcon>
                            </Group>
                        </Group>
                    </AppShell.Header>
                    <AppShell.Footer>
                        <Center h="100%" w="100%" bg={vars.colors.blue.filled} c="white">
                            <Text>
                                © 2024{" "}
                                <a href="https://github.com/hurryabit/" style={{ color: "white" }}>
                                    Martin Huschenbett
                                </a>
                            </Text>
                        </Center>
                    </AppShell.Footer>
                    <AppShell.Navbar></AppShell.Navbar>
                    <AppShell.Main h="100dvh">
                        <Main />
                    </AppShell.Main>
                </AppShell>
            </AppStateProvider>
        </MantineProvider>
    );
}
