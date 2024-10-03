import { useEffect, useMemo } from "react";
import { Box, Group, ScrollArea, Tree, TreeNodeData, useTree } from "@mantine/core";
import { IconAbc, IconChevronDown } from "@tabler/icons-react";

import syntax from "felix-wasm-bridge";

import '@mantine/code-highlight/styles.css';
import "ace-builds/css/theme/github_light_default.css";
import * as classes from "./SyntaxTree.css";

function syntaxToData(root: syntax.Node): TreeNodeData[] {
    function goElement(element: syntax.Element): TreeNodeData {
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
            label: node.kind,
            children: node.children.flatMap(goElement),
        };
    }

    function goToken(token: syntax.Token): TreeNodeData {
        return {
            value: token.id,
            label: `${token.kind} — ${token.text}`,
        };
    }

    return goNode(root).children ?? [];
}

type Props = {
    syntax?: syntax.Node;
}

export default function SyntaxTree({ syntax }: Props) {
    const data = useMemo(function () {
        return syntax !== undefined ? syntaxToData(syntax ?? false) : [];
    }, [syntax]);
    const tree = useTree();
    const { expand } = tree;

    useEffect(function () {
        for (const node of data) {
            expand(node.value);
        }
    }, [expand, data]);

    return <ScrollArea type="scroll" h="100%">
        <Box pt="xs" pb="xs" pl="md" pr="md" className="ace-github-light-default">
            <Tree
                data={data}
                tree={tree}
                levelOffset={24}
                renderNode={({ node, expanded, hasChildren, elementProps }) => (
                    <Group gap={8} {...elementProps}>
                        {hasChildren
                            ? <IconChevronDown
                                size={16}
                                style={{ transform: expanded ? 'rotate(180deg)' : 'rotate(0deg)' }}
                            />
                            : <IconAbc size={16} />
                        }
                        <span className={classes.syntaxKind}>
                            {node.label}
                            {
                                node.nodeProps && <>
                                    {" — "}<span className={node.nodeProps.className}>{node.nodeProps.text ?? ""}</span>
                                </>
                            }
                        </span>
                    </Group>
                )}
            />
        </Box>
    </ScrollArea>;
}
