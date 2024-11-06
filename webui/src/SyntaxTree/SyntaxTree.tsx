import { MouseEvent, useCallback, useEffect } from "react";
import { IconAbc, IconChevronDown } from "@tabler/icons-react";
import { Box, Group, ScrollArea, Text, Tree, useTree } from "@mantine/core";

import "@mantine/code-highlight/styles.css";
import "ace-builds/css/theme/github_light_default.css";

import { useScrollIntoView } from "@mantine/hooks";
import { useAppState, useAppStateDispatch } from "../AppState/hooks";
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
    const { elements, inspected, treeData, gotoCursor } = useAppState();
    const dispatch = useAppStateDispatch();
    const tree = useTree({ multiple: false, initialExpandedState: { "": true } });
    const { expand, toggleExpanded } = tree;
    const { scrollableRef, targetRef, scrollIntoView } = useScrollIntoView<
        HTMLDivElement,
        HTMLDivElement
    >({ duration: 200, easing: easeInOutExpo });

    useEffect(() => {
        if (inspected === null || inspected.path.length === 0) return;
        inspected.path.forEach(expand);
        setTimeout(() => scrollIntoView({ alignment: "center" }), 10);
    }, [inspected, expand, scrollIntoView]);

    const onClickChevron = useCallback(
        (event: MouseEvent<SVGSVGElement>) => {
            const node = event.currentTarget.closest<HTMLElement>("[data-value]")?.dataset.value;
            if (node === undefined) return;
            toggleExpanded(node);
        },
        [toggleExpanded],
    );

    const onClickLabel = useCallback(
        (event: MouseEvent<HTMLElement>) => {
            const node = event.currentTarget.closest<HTMLElement>("[data-value]")?.dataset.value;
            if (node === undefined) return;
            if (node === inspected?.node) {
                dispatch({ type: "inspectNodeFromTree", node: null });
            } else {
                dispatch({ type: "inspectNodeFromTree", node });
                const syntax = elements.get(node);
                if (syntax !== undefined) {
                    gotoCursor(syntax.start);
                }
            }
        },
        [elements, inspected, dispatch, gotoCursor],
    );

    const onMouseEnterLabel = useCallback(
        (event: MouseEvent<HTMLElement>) => {
            const node = event.currentTarget.closest<HTMLElement>("[data-value]")?.dataset.value;
            if (node === undefined) return;
            dispatch({ type: "setHoveredNode", node });
        },
        [dispatch],
    );

    const onMouseLeaveLabel = useCallback(
        () => dispatch({ type: "setHoveredNode", node: null }),
        [dispatch],
    );

    return (
        <ScrollArea type="scroll" h="100%" viewportRef={scrollableRef}>
            <Box pt="xs" pb="xs" pl="md" pr="md" className="ace-github-light-default">
                <Tree
                    data={treeData}
                    tree={tree}
                    levelOffset={24}
                    renderNode={({ node, expanded, hasChildren, elementProps }) => (
                        <Group
                            gap={8}
                            {...elementProps}
                            onClick={undefined}
                            style={{
                                background: "none",
                                cursor: "default",
                            }}
                        >
                            {hasChildren ? (
                                <IconChevronDown
                                    size={16}
                                    style={{
                                        cursor: "pointer",
                                        transform: expanded ? "rotate(180deg)" : "rotate(0deg)",
                                    }}
                                    onClick={onClickChevron}
                                />
                            ) : (
                                <IconAbc size={16} />
                            )}
                            <Text
                                component="span"
                                className={classes.syntaxKind}
                                data-selected={node.value === inspected?.node ? true : undefined}
                                ref={node.value === inspected?.node ? targetRef : undefined}
                                onClick={onClickLabel}
                                onMouseEnter={onMouseEnterLabel}
                                onMouseLeave={onMouseLeaveLabel}
                            >
                                {node.label}
                            </Text>
                        </Group>
                    )}
                />
            </Box>
        </ScrollArea>
    );
}
