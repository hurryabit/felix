import { style } from "@vanilla-extract/css";
import { rem } from '@mantine/core';

export const mainColumn = style({
    display: "flex",
    flexDirection: "column",
    height: "100%",
});

export const editorOutputRow = style({
    display: "flex",
    flexDirection: "row",
    flexGrow: "1",
});

export const editorPane = style({
    display: "block",
    flexGrow: 1,
});

export const outputPane = style({
    display: "block",
    flexGrow: 1,
    borderLeft: "1px solid var(--app-shell-border-color)",
});

export const problemsPane = style({
    display: "block",
    height: rem(200),
    borderTop: "1px solid var(--app-shell-border-color)",
});
