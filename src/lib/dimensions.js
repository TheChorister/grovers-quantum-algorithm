import { useEffect, useState, useLayoutEffect } from 'react';

export function useDimensions (targetRef) {
    const getDimensions = () => {
        //return { width: window.innerWidth, height: window.innerHeight };
        return {
            width: targetRef.current ? targetRef.current.offsetWidth : 0,
            height: targetRef.current ? targetRef.current.offsetHeight : 0
        };
    };

    const [dimensions, setDimensions] = useState(getDimensions);

    const handleResize = () => {
        setDimensions(getDimensions());
    };

    useEffect(() => {
        window.addEventListener("resize", handleResize);
        return () => window.removeEventListener("resize", handleResize);
    }, []);

    useEffect(() => {
        const observer = new MutationObserver(() => handleResize());
        observer.observe(targetRef.current, { attributes: true, childList: true, subtree: true });
        return () => observer.disconnect();
    }, [ targetRef ]);

    useLayoutEffect(() => {
        handleResize();
    }, []);

    return dimensions;
}
