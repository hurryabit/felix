import { useEffect, useMemo } from "react";
import { IconAbc, IconChevronDown } from "@tabler/icons-react";
import syntax from "felix-wasm-bridge";
import { Box, Group, ScrollArea, Tree, TreeNodeData, useTree } from "@mantine/core";

import "@mantine/code-highlight/styles.css";
import "ace-builds/css/theme/github_light_default.css";

import * as classes from "./SyntaxTree.css";

function syntaxToData(topLevel: syntax.Element[]): [TreeNodeData[], Map<string, syntax.Element>] {
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

    return [topLevel.map(goElement), elements];
}

type Props = {
    syntax?: syntax.Node;
    setHoveredSyntax: (element: syntax.Element | null) => void;
};

export default function SyntaxTree({ syntax, setHoveredSyntax }: Props) {
    const [data, elements] = useMemo(
        function () {
            return syntaxToData(syntax?.children ?? []);
        },
        [syntax],
    );
    const tree = useTree();
    const { expand, hoveredNode } = tree;

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
            for (const node of data) {
                expand(node.value);
            }
        },
        [expand, data],
    );

    return (
        <ScrollArea type="scroll" h="100%">
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
                            {node.label}
                        </Group>
                    )}
                />
            </Box>
        </ScrollArea>
    );
}
