import { vars } from "../theme";
import { style } from "@vanilla-extract/css";

export const syntaxKind = style({
    fontFamily: vars.fontFamilyMonospace,
    fontSize: vars.fontSizes.md,
});
