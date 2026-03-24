import React, { useRef, useMemo } from 'react';
import { useDimensions } from '../../lib/dimensions';
import * as d3 from 'd3';
import Complex from 'complex.js';
import { animated, useSpring } from '@react-spring/web';

const PADDING = 40;

function CartesianRaw ({ targets, components, size, allValues, ref }) {
    const scale = d3.scaleLinear().domain([-1.25, 1.25]).range([PADDING, size - PADDING * 2]);
    const probabilityScale = d3.scalePow().domain([-1, 1]).range([-1, 1]).exponent(0.5);
    const [ comp_key, comp_nokey ] = useMemo(() => {
        var sum_key = new Complex(0);
        var sum_nokey = new Complex(0);
        var prob_key = 0;
        var prob_nokey = 0;
        allValues.forEach(t => {
            const component = components?.find(c => c.res == t);
            if (component) {
                const { comp, prob } = component;
                if (targets.includes(t)) {
                    sum_key = sum_key.add(comp);
                    prob_key += prob;
                }
                else {
                    sum_nokey = sum_nokey.add(comp);
                    prob_nokey += prob;
                }
            }
        });
        return [ Math.sqrt(prob_key) * Math.sign(sum_key.re), Math.sqrt(prob_nokey) * Math.sign(sum_nokey.re) ];
    }, [ targets, components ]);
    const springProps = useSpring({
        from: {
            y: 0,
            x: 0,
        },
        to: {
            y: comp_key,
            x: comp_nokey,
        }
    });

    const grid = probabilityScale
        .ticks(7)
        .map((value, i) => (
            <g key={i}>
                <line
                    x1={scale(-1.25)}
                    x2={scale(1.25)}
                    y1={scale(-probabilityScale(value))}
                    y2={scale(-probabilityScale(value))}
                    fill="#808080"
                    stroke="#808080"
                    opacity={0.2}
                />
                <text
                    x={scale(-1.25) + 20}
                    y={10 * Math.sign(-value) + scale(-probabilityScale(value))}
                    textAnchor="middle"
                    alignmentBaseline="central"
                    fontSize={9}
                    fill="#808080"
                    opacity={0.8}
                >
                    {Math.abs(value) * 100}%
                </text>
            </g>
        ));
    
    return <svg width={size} height={size}>
        <defs>
            <marker
                id="arrow"
                viewBox="0 0 10 10"
                refX="5"
                refY="5"
                markerWidth="6"
                markerHeight="6"
                orient="auto-start-reverse"
            >
                <path d="M 0 0 L 10 5 L 0 10 z" />
            </marker>
        </defs>
        <circle
            r={scale(1) - scale(0)}
            cx={scale(0)}
            cy={scale(0)}
            stroke="black"
            strokeWidth="3"
            strokeOpacity="0.5"
            strokeDasharray="10,5"
            fill="none"
        ></circle>
        <g id="axes">
            {grid}
            <line
                x1={scale(0)}
                x2={scale(0)}
                y1={scale(1.25)}
                y2={scale(-1.25)}
                stroke="black"
                strokeWidth="1"
                markerEnd="url(#arrow)"
            />
            <text
                x={scale(0) + 10}
                y={scale(-1.25) + 20}
            >🔑</text>
            <line
                x1={scale(-1.25)}
                x2={scale(1.25)}
                y1={scale(0)}
                y2={scale(0)}
                stroke="black"
                strokeWidth="1"
                markerEnd="url(#arrow)"
            />
            <text
                x={scale(1.25) - 20}
                y={scale(0) + 30}
            >not 🔑</text>
        </g>
        <line
            x1={scale(0)}
            y1={scale(0)}
            x2={scale(Math.sqrt(allValues.length - 1)/Math.sqrt(allValues.length))}
            y2={scale(-1/Math.sqrt(allValues.length))}
            stroke="gray"
            strokeWidth="3"
            strokeOpacity="0.6"
            strokeDasharray="10,5"
        />
        <animated.line
            x1={scale(0)}
            y1={scale(0)}
            x2={springProps?.x.to(x => scale(x / 1.03))}
            y2={springProps?.y.to(y => scale(-y / 1.03))}
            stroke="black"
            strokeWidth="3"
            markerEnd="url(#arrow)"
        />
    </svg>
}

export default function Cartesian ({ targets, components, allValues, parentRef }) {
    const { width, height } = useDimensions(parentRef);
    return <CartesianRaw targets={targets} components={components} size={Math.min(width, height)} allValues={allValues} />
}