import { useEffect, useMemo, useState } from "react";
import { IconAbc, IconChevronDown } from "@tabler/icons-react";
import syntax from "felix-wasm-bridge";
import { Box, Group, ScrollArea, Tree, TreeNodeData, useTree } from "@mantine/core";

import "@mantine/code-highlight/styles.css";
import "ace-builds/css/theme/github_light_default.css";

import { useScrollIntoView } from "@mantine/hooks";
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
function syntaxToData(root: syntax.Element): [TreeNodeData[], Map<string, syntax.Element>] {
    const elements = new Map();

    function goElement(element: syntax.Element): TreeNodeData {
        elements.set(element.id, element);
        switch (element.tag) {
            case "NODE":
                return goNode(element);
            case "TOKEN":
                return goToken(element);
        }
    }

    function goNode(node: syntax.Node): TreeNodeData {
        return {
            value: node.id,
            label: <span className={classes.syntaxKind}>{node.kind}</span>,
            children: node.children.map(goElement),
        };
    }

    function goToken(token: syntax.Token): TreeNodeData {
        return {
            value: token.id,
            label: (
                <span className={classes.syntaxKind}>
                    {token.kind} — {token.text}
                </span>
            ),
        };
    }

    return [[goElement(root)], elements];
}

function before(x: syntax.SrcLoc, y: syntax.SrcLoc): boolean {
    return x.line < y.line || (x.line === y.line && x.column <= y.column);
}

function findCursed(
    element: syntax.Element,
    cursor: syntax.SrcLoc,
    expand: (value: string) => void,
): syntax.Element {
    // eslint-disable-next-line no-constant-condition
    while (true) {
        if (element.tag === "TOKEN") {
            return element;
        }
        expand(element.id);
        // TODO(MH): Use binary search for large counts of children.
        const child = element.children.findLast((x) => before(x.start, cursor));
        if (child === undefined) {
            return element;
        }
        if (child.tag === "NODE" && before(cursor, child.start)) {
            expand(child.id);
            return child;
        }
        element = child;
    }
}

type Props = {
    syntax?: syntax.Element;
    cursor?: syntax.SrcLoc;
    setHoveredSyntax: (element: syntax.Element | null) => void;
    setCursedSyntax: (element: syntax.Element | null) => void;
};

export default function SyntaxTree({ syntax, cursor, setHoveredSyntax, setCursedSyntax }: Props) {
    const [data, elements] = useMemo(
        function () {
            return syntax !== undefined ? syntaxToData(syntax) : [[], new Map()];
        },
        [syntax],
    );
    const tree = useTree();
    const { expand, hoveredNode } = tree;
    const [cursed, setCursed] = useState<syntax.Element>();
    const { scrollableRef, targetRef, scrollIntoView } = useScrollIntoView<
        HTMLDivElement,
        HTMLDivElement
    >({ duration: 200, easing: easeInOutExpo });

    // NOTE(MH): This is ugly but I don't know how to do better with the
    // current Tree API.
    useEffect(
        function () {
            setHoveredSyntax(hoveredNode !== null ? (elements.get(hoveredNode) ?? null) : null);
        },
        [elements, hoveredNode, setHoveredSyntax],
    );

    useEffect(
        function () {
            let cursed: syntax.Element | undefined;
            if (syntax) {
                expand(syntax.id);
                cursed = cursor ? findCursed(syntax, cursor, expand) : syntax;
            }
            setCursed(cursed);
            setCursedSyntax(cursed ?? null);
        },
        [syntax, cursor, expand, setCursedSyntax],
    );

    useEffect(
        function () {
            scrollIntoView({ alignment: "center" });
        },
        [cursed, scrollIntoView],
    );

    return (
        <ScrollArea type="scroll" h="100%" viewportRef={scrollableRef}>
            <Box pt="xs" pb="xs" pl="md" pr="md" className="ace-github-light-default">
                <Tree
                    data={data}
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
                                className={node.value === cursed?.id ? classes.cursed : undefined}
                                ref={node.value === cursed?.id ? targetRef : undefined}
                            >
                                {node.label}
                            </Box>
                        </Group>
                    )}
                />
            </Box>
        </ScrollArea>
    );
}
