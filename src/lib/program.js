import React, { useRef } from 'react';
import { Program } from 'quantum';
import useForceUpdate from 'use-force-update';
import Complex from 'complex.js';

export default function useProgram () {
    const progRef = useRef(null);
    if (progRef.current == null) {
        progRef.current = new Program();
    }
    const forceUpdate = useForceUpdate();
    return {
        get max_bits() {
            return Program.get_max_bits();
        },
        add: (gate) => {
            progRef.current.add(gate);
            forceUpdate();
        },
        clear: () => {
            progRef.current.clear();
            forceUpdate();
        },
        get_probability: async (res) => {
            const prob = progRef.current.get_probability(res);
            return prob;
        },
        run: async () => {
            const res = progRef.current.run();
            return res;
        },
        get_gates: () => {
            const gates = progRef.current.get_gates();
            return gates;
        },
        get_component: async (res) => {
            const comp = progRef.current.get_component(res);
            return new Complex({ re: comp.re, im: comp.im });
        }
    };
}
