import { MouseEvent, useCallback } from "react";
import { IconCircleX } from "@tabler/icons-react";
import { Box, List, ScrollArea } from "@mantine/core";
import { useAppState } from "../AppState/hooks";
import * as classes from "./Problems.css";

export default function ProblemsPane() {
    const { problems, gotoCursor } = useAppState();
    const onProblemClick = useCallback(
        (event: MouseEvent<HTMLLIElement>) => {
            const index = parseInt(event.currentTarget.dataset.index ?? "");
            if (isNaN(index) || index < 0 || index >= problems.length) {
                console.error(`Cannot handle click on problem: ${event}`);
                return;
            }
            gotoCursor(problems[index].start);
        },
        [problems, gotoCursor],
    );

    return (
        <ScrollArea type="scroll" h="100%">
            <Box pt="xs">
                <List
                    className={classes.problemsList}
                    center
                    icon={<IconCircleX className={classes.problemIcon} />}
                >
                    {problems.map(({ start, message, source }, i) => {
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
