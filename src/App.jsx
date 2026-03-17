import React, { useState, useEffect, useMemo } from 'react';
import { Gate, GateType } from 'quantum';
import useProgram from './lib/program.js';

function into_bits(n, max_bits=8) {
    var bits = [];
    for (let i = 0; i < max_bits; i++) {
        if ((1 << i) & n > 0) {
            bits.push(i);
        }
    }
    return bits;
}

export default function App ({ bits }) {
    const program = useProgram();
    const [target, setTarget] = useState(0);
    const [components, setComponents] = useState([]);
    const allBits = useMemo(() => Array(bits).fill(0).map((_, i) => i), [ bits ]);
    const allValues = useMemo(() => Array(2**bits).fill(0).map((_, i) => i), [ bits ]);

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

    return <main>
        <input value={target} onChange={e => setTarget(e.target.value)} type="number" min={0} max={2**bits - 1} />
        <button onClick={() => {
            allBits.forEach(bit => program.add(new Gate(GateType.Hadamard, [ bit ])));
        }}>Initialise</button>
        <button onClick={() => {
            program.add(new Gate(GateType.GroverOracle, into_bits(target, bits)));
        }}>Apply Oracle</button>
        <button onClick={() => {
            program.add(new Gate(GateType.GroverDiffuser, allBits));
        }}>Apply Diffuser</button>
        { components.map(({ res, comp, prob }) => {
            return <div key={res}>
                {res}: {Math.round(prob * 10000)/ 100}% chance; phase: { comp.arg()/Math.PI } pi
            </div>
        }) }
    </main>;
}
