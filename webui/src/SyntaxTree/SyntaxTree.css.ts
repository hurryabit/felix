import { globalStyle, style } from "@vanilla-extract/css";
import { vars } from "../theme";

export const syntaxKind = style({
    fontFamily: vars.fontFamilyMonospace,
    fontSize: vars.fontSizes.md,
    paddingLeft: vars.radius.sm,
    paddingRight: vars.radius.sm,
    borderRadius: vars.radius.sm,
    ":hover": {
        background: vars.colors.primaryColors.lightHover,
    },
});

export const cursed = style({});

globalStyle(`${cursed} ${syntaxKind}`, {
    background: vars.colors.green.lightHover,
});
