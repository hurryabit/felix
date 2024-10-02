import { MouseEvent, useCallback } from "react";
import { Box, List, ScrollArea } from "@mantine/core";
import { IconCircleX } from "@tabler/icons-react";

import { Problem } from "felix-wasm-bridge";

import * as classes from "./Problems.css";

type Props = {
    problems: Problem[];
}

export default function ProblemsPane({problems}: Props) {
    const onProblemClick = useCallback(function onClick(event: MouseEvent<HTMLLIElement>) {
        alert(`Clicked on problem ${event.currentTarget.dataset.index}.`);
    }, []);

    return <ScrollArea h="100%" type="auto">
        <Box pt="xs">
            <List
                className={classes.problemsList}
                center
                icon={<IconCircleX className={classes.problemIcon} />}
            >
                {problems.map(function ({start, message, source}, i) {
                    const { line, column } = start;
                    return <List.Item key={i} data-index={i} onClick={onProblemClick}>
                        {message} <span className={classes.locAnn}>— {source} [Ln {line}, Col {column}]</span>
                    </List.Item>
                })}
            </List>
        </Box>
    </ScrollArea>;
}
