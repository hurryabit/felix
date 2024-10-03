import { style } from "@vanilla-extract/css";
import { rem } from "@mantine/core";
import { vars } from "../theme";

export const errorMarker = style({
    position: "absolute",
    borderRadius: 0,
    borderBottom: "dotted 2px red",
});

export const highlightMarker = style({
    position: "absolute",
    background: vars.colors.primaryColors.lightHover,
    borderRadius: rem(2),
});
