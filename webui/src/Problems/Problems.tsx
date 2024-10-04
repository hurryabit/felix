import { MouseEvent, useCallback } from "react";
import { IconCircleX } from "@tabler/icons-react";
import { Problem } from "felix-wasm-bridge";
import { Box, List, ScrollArea } from "@mantine/core";
import * as classes from "./Problems.css";

type Props = {
    problems: Problem[];
    onSelect: (problem: Problem) => void;
};

export default function ProblemsPane({ problems, onSelect }: Props) {
    const onProblemClick = useCallback(
        function onClick(event: MouseEvent<HTMLLIElement>) {
            const index = parseInt(event.currentTarget.dataset.index ?? "");
            if (isNaN(index) || index < 0 || index >= problems.length) {
                console.error(`Cannot handle click on problem: ${event}`);
                return;
            }
            const problem = problems[index];
            onSelect(problem);
        },
        [problems, onSelect],
    );

    return (
        <ScrollArea type="scroll" h="100%">
            <Box pt="xs">
                <List
                    className={classes.problemsList}
                    center
                    icon={<IconCircleX className={classes.problemIcon} />}
                >
                    {problems.map(function ({ start, message, source }, i) {
                        const { line, column } = start;
                        return (
                            <List.Item key={i} data-index={i} onClick={onProblemClick}>
                                {message}{" "}
                                <span className={classes.locAnn}>
                                    — {source} [Ln {line}, Col {column}]
                                </span>
                            </List.Item>
                        );
                    })}
                </List>
            </Box>
        </ScrollArea>
    );
}
