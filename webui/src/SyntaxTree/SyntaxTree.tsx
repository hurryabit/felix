import { useEffect } from "react";
import { IconAbc, IconChevronDown } from "@tabler/icons-react";
import { Box, Group, ScrollArea, Tree, useTree } from "@mantine/core";

import "@mantine/code-highlight/styles.css";
import "ace-builds/css/theme/github_light_default.css";

import { useScrollIntoView } from "@mantine/hooks";
import { useAppState, useAppStateDispatch } from "../AppState";
import * as classes from "./SyntaxTree.css";

function easeInOutExpo(x: number): number {
    return x === 0
        ? 0
        : x === 1
          ? 1
          : x < 0.5
            ? Math.pow(2, 20 * x - 10) / 2
            : (2 - Math.pow(2, -20 * x + 10)) / 2;
}

export default function SyntaxTree() {
    const { cursedSyntax, cursedPath, treeData } = useAppState();
    const dispatch = useAppStateDispatch();
    const tree = useTree();
    const { expand, hoveredNode } = tree;
    const { scrollableRef, targetRef, scrollIntoView } = useScrollIntoView<
        HTMLDivElement,
        HTMLDivElement
    >({ duration: 200, easing: easeInOutExpo });

    // NOTE(MH): This is ugly but I don't know how to do better with the
    // current Tree API.
    useEffect(
        function () {
            dispatch({ type: "setHoveredNode", hoveredNode });
        },
        [dispatch, hoveredNode],
    );

    useEffect(
        function () {
            for (const node of cursedPath) {
                expand(node);
            }
            scrollIntoView({ alignment: "center" });
        },
        [cursedSyntax, cursedPath, expand, scrollIntoView],
    );

    return (
        <ScrollArea type="scroll" h="100%" viewportRef={scrollableRef}>
            <Box pt="xs" pb="xs" pl="md" pr="md" className="ace-github-light-default">
                <Tree
                    data={treeData}
                    tree={tree}
                    levelOffset={24}
                    renderNode={({ node, expanded, hasChildren, elementProps }) => (
                        <Group gap={8} {...elementProps}>
                            {hasChildren ? (
                                <IconChevronDown
                                    size={16}
                                    style={{
                                        transform: expanded ? "rotate(180deg)" : "rotate(0deg)",
                                    }}
                                />
                            ) : (
                                <IconAbc size={16} />
                            )}
                            <Box
                                className={
                                    node.value === cursedSyntax?.id ? classes.cursed : undefined
                                }
                                ref={node.value === cursedSyntax?.id ? targetRef : undefined}
                            >
                                <span className={classes.syntaxKind}>{node.label}</span>
                            </Box>
                        </Group>
                    )}
                />
            </Box>
        </ScrollArea>
    );
}
