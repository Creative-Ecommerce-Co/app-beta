// project imports

// action - state management
import config from '../../config';
import * as actionTypes from './actions';

export interface CustomizationState {
    isOpen: string[];
    defaultId: string;
    fontFamily: string;
    borderRadius: number;
    opened: boolean;
}

export const initialState: CustomizationState = {
    isOpen: [], // for active default menu
    defaultId: 'default',
    fontFamily: config.fontFamily,
    borderRadius: config.borderRadius,
    opened: true
};

// ==============================|| CUSTOMIZATION REDUCER ||============================== //

interface Action {
    type: string;
    id?: string;
    opened?: boolean;
    fontFamily?: string;
    borderRadius?: number;
}

const customizationReducer = (state: CustomizationState = initialState, action: Action): CustomizationState => {
    let id;
    switch (action.type) {
        case actionTypes.MENU_OPEN:
            id = action.id;
            return {
                ...state,
                isOpen: id ? [id] : []
            };
        case actionTypes.SET_MENU:
            return {
                ...state,
                opened: action.opened ?? state.opened
            };
        case actionTypes.SET_FONT_FAMILY:
            return {
                ...state,
                fontFamily: action.fontFamily ?? state.fontFamily
            };
        case actionTypes.SET_BORDER_RADIUS:
            return {
                ...state,
                borderRadius: action.borderRadius ?? state.borderRadius
            };
        default:
            return state;
    }
};

export default customizationReducer;