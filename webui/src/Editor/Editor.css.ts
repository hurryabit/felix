import { globalStyle, style } from "@vanilla-extract/css";
import { rem } from "@mantine/core";
import { vars } from "../theme";

export const errorMarker = style({
    position: "absolute",
    borderRadius: 0,
    borderBottom: "dotted 2px red",
});

export const inspectedMarker = style({
    position: "absolute",
    borderRadius: 0,
    borderBottom: `solid 2px ${vars.colors.primary}`,
});

export const hoveredMarker = style({
    position: "absolute",
    background: vars.colors.primaryColors.lightHover,
    borderRadius: rem(2),
});

export const selectionMarker = style({
    position: "absolute",
    background: vars.colors.yellow.lightHover,
    borderRadius: rem(2),
});

globalStyle(".ace_editor .ace_marker-layer :is(.ace_bracket, .ace_selection)", {
    display: "none",
});
