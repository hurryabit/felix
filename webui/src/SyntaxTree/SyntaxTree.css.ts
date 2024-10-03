import { vars } from "../theme";
import { style } from "@vanilla-extract/css";

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
