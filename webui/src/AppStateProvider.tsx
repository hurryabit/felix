import { ReactNode, useReducer } from "react";
import { DispatchContext, init, INITIAL_STATE, reducer, StateContext } from "./AppState";

type Props = { children: ReactNode };

export default function AppStateProvider({ children }: Props) {
    const [state, dispatch] = useReducer(reducer, INITIAL_STATE, init);

    return (
        <StateContext.Provider value={state}>
            <DispatchContext.Provider value={dispatch}>{children}</DispatchContext.Provider>
        </StateContext.Provider>
    );
}
