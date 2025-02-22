import { useEffect, useRef, MutableRefObject } from 'react';

// ==============================|| ELEMENT REFERENCE HOOKS  ||============================== //

const useScriptRef = (): MutableRefObject<boolean> => {
    const scripted = useRef(true);

    useEffect(() => {
        return () => {
            scripted.current = false;
        };
    }, []);

    return scripted;
};

export default useScriptRef;