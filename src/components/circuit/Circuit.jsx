import React, { useEffect, useMemo, useRef, useState } from 'react';
import { useProgramLayers } from '../../lib/program.js';
import { GateType, Gate as DataGate } from 'quantum';
import { Gate, IdentityGate, XGate } from './Gate.jsx';
import { useWindowDimensions } from '../../lib/dimensions.js';
import Probability from '../probability/Probability.jsx';

import styles from './Circuit.css';

export function RenderLayer({ layerN, layer, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    return <g data-layer-n={layerN} style={{ transform: `translateX(${layerN * gateSpacing}px)` }}>
        { layer.gates.map((g, i) => <Gate key={i} gate={g} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={bit => onClickGate(i, bit)} />) }
    </g>
}

export function GatesFromProgram({ program, wireSpacing, gateSpacing, gateSize, bits, onClickGate }) {
    var gatesI = 0;
    return <g>
        <g id="wires">
            { Array(bits).fill(null).map((_, i) => <line
                key={i}
                x1={0}
                x2={gateSpacing * (program.layers.length + 1)}
                y1={wireSpacing / 2 + wireSpacing * i}
                y2={wireSpacing / 2 + wireSpacing * i}
                stroke="black"
                strokeWidth="2"
            />) }
        </g>
        { program.layers.map((l, i) => {
            gatesI += l.gates.length;
            const copy = parseInt(`${gatesI}`);
            return <RenderLayer key={i} layer={l} layerN={i} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={(i, bit) => onClickGate(i + copy - l.gates.length, bit)} />
        }) }
    </g>
}

export default function Circuit ({ bits, visible }) {
    const program = useProgramLayers();

    useEffect(() => {
        program.add(new DataGate(GateType.Hadamard, [0]));
        program.add(new DataGate(GateType.Hadamard, [2]));
        program.add(new DataGate(GateType.Hadamard, [3]));
        program.add(new DataGate(GateType.CNOT, [0, 2]));
        program.add(new DataGate(GateType.PauliX, [3]));
        program.add(new DataGate(GateType.PauliZ, [3]));
        return () => program.clear();
    }, [ ]);

    const allBits = useMemo(() => Array(bits).fill(0).map((_, i) => i), [ bits ]);
    const allValues = useMemo(() => Array(2**bits).fill(0).map((_, i) => i), [ bits ]);
    const [components, setComponents] = useState([]);

    useEffect(() => {
        (async function () {
            setComponents(await Promise.all(
                allValues.map(async x => ({
                    res: x,
                    comp: await program.get_component(x),
                    prob: await program.get_probability(x)
                })
            )));
        })()
    }, [ JSON.stringify(program.get_gates().map(g => ({ type: g.gate_type, bits: g.bits }))), bits ]);

    const { width } = useWindowDimensions();

    const ref = useRef(null);

    return <main>
        <svg width={width} height={bits * 50}>
            <GatesFromProgram program={program} wireSpacing={50} gateSpacing={50} gateSize={30} bits={bits} onClickGate={(i, bit) => program.remove(i)/*program.insert(new DataGate(GateType.Hadamard, [ bit ]), i)*/} />
        </svg>
        <div ref={ref} className={styles.graphContainer}><Probability components={program && components || []} parentRef={ref} dependencies={[ visible ]} /></div>
    </main>
}
