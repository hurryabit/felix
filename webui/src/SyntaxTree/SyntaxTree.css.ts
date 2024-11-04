import { style } from "@vanilla-extract/css";
import { vars } from "../theme";

export const syntaxKind = style({
    fontFamily: vars.fontFamilyMonospace,
    fontSize: vars.fontSizes.md,
    lineHeight: 1,
    padding: [vars.radius.sm, vars.radius.md, vars.radius.sm, vars.radius.md],
    borderRadius: vars.radius.md,
    cursor: "pointer",
    ":hover": {
        background: vars.colors.primaryColors[1],
    },
    selectors: {
        "&[data-selected]": {
            color: vars.colors.white,
            background: vars.colors.primaryColors.filled,
        },
    },
});
