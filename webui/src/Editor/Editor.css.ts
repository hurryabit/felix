import { globalStyle, style } from "@vanilla-extract/css";
import { rem } from "@mantine/core";
import { vars } from "../theme";

export const errorMarker = style({
    position: "absolute",
    borderRadius: 0,
    borderBottom: "dotted 2px red",
});

export const hoveredMarker = style({
    position: "absolute",
    background: vars.colors.primaryColors.lightHover,
    borderRadius: rem(2),
});

export const cursedMarker = style({
    position: "absolute",
    background: vars.colors.green.lightHover,
    borderRadius: rem(2),
});

export const selectionMarker = style({
    position: "absolute",
    background: vars.colors.yellow.lightHover,
    borderRadius: rem(2),
});

globalStyle(".ace_editor .ace_marker-layer .ace_bracket", {
    display: "none",
});

globalStyle(".ace_editor .ace_marker-layer .ace_selection", {
    display: "none",
});

globalStyle(".ace_editor .ace_marker-layer .ace_selected-word", {
    display: "none",
});
