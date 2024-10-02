import { Box, Group, Tree, TreeNodeData, useTree } from "@mantine/core";
import { IconAbc, IconChevronDown } from "@tabler/icons-react";

import "ace-builds/css/theme/github_light_default.css"
import * as classes from "./SyntaxTree.css";
import { useEffect } from "react";

type Props = {
    data: TreeNodeData[];
}

export default function SyntaxTreeView({data}: Props) {
    const tree = useTree();
    const { expandAllNodes } = tree;

    useEffect(function() {
        expandAllNodes();
    }, [expandAllNodes, data]);

    return <Box pt="xs" pb="xs" pl="md" pr="md" className="ace-github-light-default">
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
    </Box>;
}
