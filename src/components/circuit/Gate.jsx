import React from 'react';
import { GateType } from 'quantum';

function BoxGate({ bit, wireSpacing, gateSpacing, gateSize, name, onClickGate }) {
    return <g data-gate-type={name} data-gate-bit={bit} style={{ transform: `translateY(${bit * wireSpacing}px)` }}>
        <rect
            x={(gateSpacing - gateSize) / 2}
            y={(wireSpacing - gateSize) / 2}
            width={gateSize}
            height={gateSize}
            fill="white"
            stroke="black"
            strokeWidth="2"
            onClick={() => onClickGate(bit)}
        />
        <text x={ gateSpacing / 2 } y={ gateSpacing / 2 } dominantBaseline="middle" textAnchor="middle" onClick={() => onClickGate(bit)}>{ name }</text>
    </g>
}

export function IdentityGate({ bit, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    return <BoxGate bit={bit} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} name="I" onClickGate={onClickGate} />
}

export function ZGate({ bit, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    return <BoxGate bit={bit} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} name="Z" onClickGate={onClickGate} />
}

export function YGate({ bit, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    return <BoxGate bit={bit} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} name="Y" onClickGate={onClickGate} />
}

export function HadamardGate({ bit, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    return <BoxGate bit={bit} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} name="H" onClickGate={onClickGate} />
}

export function PhaseGate({ bit, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    return <BoxGate bit={bit} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} name="S" onClickGate={onClickGate} />
}

export function TGate({ bit, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    return <BoxGate bit={bit} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} name="T" onClickGate={onClickGate} />
}

export function XGate({ bit, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    return <g data-gate-type="X" data-gate-bit={bit} style={{ transform: `translateY(${bit * wireSpacing}px)` }}>
        <circle
            cx={gateSpacing / 2}
            cy={wireSpacing / 2}
            r={gateSize / 2}
            stroke="black"
            strokeWidth="2"
            fill="white"
            onClick={() => onClickGate(bit)}
        />
        <line
            x1={gateSpacing / 2}
            x2={gateSpacing / 2}
            y1={(wireSpacing - gateSize) / 2}
            y2={wireSpacing - (wireSpacing - gateSize) / 2}
            stroke="black"
            strokeWidth="2"
            onClick={() => onClickGate(bit)}
        />
        <line
            x1={gateSpacing / 2 - gateSize / 2}
            x2={gateSpacing / 2 + gateSize / 2}
            y1={wireSpacing / 2}
            y2={wireSpacing / 2}
            stroke="black"
            strokeWidth="2"
            onClick={() => onClickGate(bit)}
        />
    </g>;
}

export function ControlBit({ bit, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    return <g data-gate-type="control" data-gate-bit={bit} style={{ transform: `translateY(${bit * wireSpacing}px)` }}>
        <circle
            cx={gateSpacing / 2}
            cy={wireSpacing / 2}
            r={5}
            stroke="none"
            fill="black"
            onClick={() => onClickGate(bit)}
        />
    </g>
}

export function WireConnection({ bits, wireSpacing, gateSpacing, gateSize }) {
    return <g data-gate-type="wire-connection" data-gate-bit-upper={Math.min(...bits)} data-gate-bit-lower={Math.max(...bits)}>
        <line
            x1={gateSpacing  / 2}
            x2={gateSpacing / 2}
            y1={wireSpacing / 2 + wireSpacing * Math.min(...bits)}
            y2={wireSpacing / 2 + wireSpacing * Math.max(...bits)}
            stroke="black"
            strokeWidth="2"
        />
    </g>;
}

export function CNOTGate({ control, target, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    return <g data-gate-type="CNOT">
        <WireConnection bits={[ control, target ]} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} />
        <ControlBit bit={control} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={onClickGate} />
        <XGate bit={target} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={onClickGate} />
    </g>
}

export function Gate({ gate, wireSpacing, gateSpacing, gateSize, onClickGate }) {
    switch (gate.gate_type) {
        case GateType.Hadamard:
            return <HadamardGate bit={gate.bits[0]} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={onClickGate} />;
        case GateType.Identity:
            return <IdentityGate bit={gate.bits[0]} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={onClickGate} />;
        case GateType.PauliZ:
            return <ZGate bit={gate.bits[0]} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={onClickGate} />;
        case GateType.PauliX:
            return <XGate bit={gate.bits[0]} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={onClickGate} />;
        case GateType.PauliY:
            return <YGate bit={gate.bits[0]} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={onClickGate} />;
        case GateType.TGate:
            return <TGate bit={gate.bits[0]} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={onClickGate} />;
        case GateType.PhaseGate:
            return <PhaseGate bit={gate.bits[0]} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={onClickGate} />;
        case GateType.CNOT:
            return <CNOTGate target={gate.bits[0]} control={gate.bits[1]} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} onClickGate={onClickGate} />;
        default:
            return <BoxGate bit={gate.bits[0]} wireSpacing={wireSpacing} gateSpacing={gateSpacing} gateSize={gateSize} name="?" onClickGate={onClickGate} />;
    }
}
