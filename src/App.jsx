import React, { useState, useEffect, useMemo, useRef } from 'react';
import { Gate, GateType } from 'quantum';
import useProgram from './lib/program.js';
import { Parser } from 'safe-expr-eval';
import Probability from './components/probability/Probability.jsx';

function into_bits(n, max_bits=8) {
    var bits = [];
    for (let i = 0; i < max_bits; i++) {
        if (((1 << i) & n) > 0) {
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
    const parserRef = useRef(null);
    if (parserRef.current == null) {
        parserRef.current = new Parser();
        parserRef.current.functions.max = Math.max;
        parserRef.current.functions.min = Math.min;
        parserRef.current.functions.round = Math.round;
        parserRef.current.functions.pow = Math.pow;
        parserRef.current.functions.exp = Math.exp;
        parserRef.current.functions.ln = Math.log;
        parserRef.current.functions.log10 = Math.log10;
        parserRef.current.functions.log2 = Math.log2;
        parserRef.current.functions.log = (base, value) => Math.log(value) / Math.log(base);
        parserRef.current.consts.PI = Math.PI;
        parserRef.current.consts.e = Math.e;
    }
    const parser = parserRef.current;
    const [expr, setExpr] = useState('false');
    
    const [ oracleType, setOracleType ] = useState('value');

    useEffect(() => {
        if (oracleType == 'value') {
            setExpr(`x == ${target}`);
        }
    }, [ oracleType, target ]);

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
        <div>
            <input type="checkbox" name="oracleType" defaultChecked={oracleType == 'value'} onChange={e => setOracleType(e.target.checked ? 'value' : 'expr')} />
            <label htmlFor="oracleType">Use known target?</label>
        </div>
        <div>
            {
                oracleType == 'value' ? 
                   <> x = <input value={target} onChange={e => setTarget(e.target.value)} type="number" min={0} max={2**bits - 1} /> </>
                    : <>{'(x) => '}<input value={expr} onChange={e => setExpr(e.target.value)} /></>
            }
        </div>
        <button onClick={() => {
            allBits.forEach(bit => program.add(new Gate(GateType.Hadamard, [ bit ])));
        }}>Initialise</button>
        <button onClick={() => {
            if (oracleType == 'value') program.add(new Gate(GateType.GroverOracle, into_bits(target, bits)));
            else {
                const expression = parser.parse(expr);
                allValues.forEach(v => {
                    if (expression.evaluate({ x: v })) {
                        program.add(new Gate(GateType.GroverOracle, into_bits(v, bits)));
                    }
                });
            }
        }}>Apply Oracle</button>
        <button onClick={() => {
            program.add(new Gate(GateType.GroverDiffuser, allBits));
        }}>Apply Diffuser</button>
        <button onClick={() => program.clear()}>Reset</button>
        <Probability components={components} />
    </main>;
}
