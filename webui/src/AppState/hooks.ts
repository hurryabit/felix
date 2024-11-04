import { Dispatch, useContext } from "react";
import { DispatchContext, StateContext } from "./AppStateProvider";
import type * as reducer from "./reducer";

export type State = reducer.State;

export type Action = reducer.Action;

export function useAppState(): State {
    return useContext(StateContext);
}

export function useAppStateDispatch(): Dispatch<Action> {
    const dispatch = useContext(DispatchContext);
    if (!dispatch) {
        console.error("Using useAppStateDispatch outside of AppStateProvider.");
    }
    return dispatch!;
}
