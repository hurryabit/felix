import { createContext, Dispatch, ReactNode, useReducer } from "react";
import { Action, State, init, INITIAL_STATE, reducer } from "./reducer";

type Props = { children: ReactNode };

export const StateContext = createContext<State>(INITIAL_STATE);
export const DispatchContext = createContext<Dispatch<Action> | undefined>(undefined);

export default function AppStateProvider({ children }: Props) {
    const [state, dispatch] = useReducer(reducer, INITIAL_STATE, init);

    return (
        <StateContext.Provider value={state}>
            <DispatchContext.Provider value={dispatch}>{children}</DispatchContext.Provider>
        </StateContext.Provider>
    );
}
