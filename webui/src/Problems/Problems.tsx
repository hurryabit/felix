import { Box, List, ScrollArea } from "@mantine/core";

import * as classes from "./Problems.css";
import { IconCircleX } from "@tabler/icons-react";
import { MouseEvent, useCallback } from "react";

export default function ProblemsPane() {
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
                {[1, 2, 3, 4, 5, 6, 7, 8, 9, 10].map(function (i) {
                    return <List.Item key={i} data-index={i} onClick={onProblemClick}>
                        Found FOO{i}, expected BAR{i} <span className={classes.locAnn}>[Ln {i}, Col 1]</span>
                    </List.Item>
                })}
            </List>
        </Box>
    </ScrollArea>;
}
