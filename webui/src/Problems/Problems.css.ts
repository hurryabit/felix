import { globalStyle, style } from "@vanilla-extract/css";
import { rem } from "@mantine/core";
import { vars } from "../theme";

export const problemsList = style({});

globalStyle(`${problemsList} li`, {
    paddingLeft: vars.spacing.md,
    paddingTop: rem(2),
    paddingBottom: rem(2),
});

globalStyle(`${problemsList} li:hover`, {
    backgroundColor: vars.colors.defaultHover,
    cursor: "pointer",
});

export const locAnn = style({
    color: vars.colors.placeholder,
    fontSize: vars.fontSizes.sm,
});

export const problemIcon = style({
    color: vars.colors.red.filled,
    height: rem(22),
    width: rem(22),
});
