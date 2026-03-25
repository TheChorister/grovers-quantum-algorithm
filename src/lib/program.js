import React, { useRef, useMemo } from 'react';
import { Program, Gate, GateType } from 'quantum';
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
        insert: (gate, i) => {
            progRef.current.insert(i, gate);
            forceUpdate();
        },
        remove: (i) => {
            progRef.current.remove(i);
            forceUpdate();
        },
        clear: () => {
            progRef.current.clear();
            forceUpdate();
        },
        get_probability: async (res) => {
            return progRef.current.get_probability(res);
        },
        run: async () => {
            return progRef.current.run();
        },
        get_gates: () => {
            return progRef.current.get_gates();
        },
        get_component: async (res) => {
            const comp = progRef.current.get_component(res);
            return new Complex({ re: comp.re, im: comp.im });
        }
    };
}


export class Layer {
    constructor (gates, bitRange = new Set()) {
        this.gates = gates;
        this.bitRange = bitRange;
    }
}

export function useProgramLayers() {
    const program = useProgram();
    const layers = useRef([]);
    return {
        get max_bits() {
            return program.max_bits;
        },
        add(gate) {
            this.add_raw(gate);
            program.add(new Gate(gate.gate_type, gate.bits));
        },
        add_raw(gate) {
            switch (gate.gate_type) {
                case GateType.GroverDiffuser:
                case GateType.GroverOracle:
                    layers.current.push(new Layer([ gate ], new Set(Array(program.max_bits).map((_, i) => i))));
                    break;
                case GateType.CNOT:
                case GateType.CZ:
                case GateType.Hadamard:
                case GateType.Identity:
                case GateType.PauliX:
                case GateType.PauliY:
                case GateType.PauliZ:
                case GateType.Phase:
                case GateType.Swap:
                case GateType.T:
                    const min_bit = Math.min(...gate.bits);
                    const max_bit = Math.max(...gate.bits);
                    const range = new Set(new Array(max_bit - min_bit + 1).fill(0).map((_, i) => min_bit + i));
                    if (layers.current.length > 0 && range.intersection(layers.current[layers.current.length - 1].bitRange).size == 0) {
                        layers.current[layers.current.length - 1].bitRange = layers.current[layers.current.length - 1].bitRange.union(range);
                        layers.current[layers.current.length - 1].gates.push(gate);
                    } else {
                        layers.current.push(new Layer([ gate ], range));
                    }
            }
        },
        remove(i) {
            if (program.get_gates().length <= i) {
                return;
            }
            var gateI = 0;
            var layerI = 0;
            for (; layerI < layers.current.length; layerI++) {
                if (gateI <= i && i < gateI + layers.current[layerI].gates.length) {
                    break;
                }
                gateI += layers.current[layerI].gates.length;
            }
            layers.current = layers.current.slice(0, layerI);
            program.remove(i);
            const gates = program.get_gates();
            gates.slice(gateI).forEach(g => {
                this.add_raw(g);
            });
        },
        insert(gate, i) {
            var gateI = 0;
            var layerI = 0;
            for (; layerI < layers.current.length; layerI++) {
                if (gateI <= i && i < gateI + layers.current[layerI].gates.length) {
                    break;
                }
                gateI += layers.current[layerI].gates.length;
            }
            layers.current = layers.current.slice(0, layerI);
            program.insert(gate, i);
            const gates = program.get_gates();
            gates.slice(gateI).forEach(g => {
                this.add_raw(g);
            });
        },
        clear() {
            program.clear();
            layers.current = [];
        },
        async get_probability(res) {
            return await program.get_probability(res);
        },
        async run() {
            return await program.run();
        },
        get_gates() {
            return program.get_gates();
        },
        async get_component(res) {
            return await program.get_component(res);
        },
        layers: layers.current
    }
}
