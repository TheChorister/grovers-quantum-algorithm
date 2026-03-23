import React, { useRef, useMemo } from 'react';

import * as d3 from 'd3';
import { useDimensions } from '../../lib/dimensions';
import { animated, useSpring } from '@react-spring/web';

console.log(d3);

function ProbabilityItem ({ res, prob, comp, xScale, yScale, boundsHeight }) {
    const value = prob * Math.sign(comp.re);
    const barWidth = xScale.bandwidth();
    const [ x, y ] = [ xScale(res), Math.min(yScale(0), yScale(value)) ];
    const isNegative = yScale(0) - yScale(value) < 0;
    const colorScale = d3.scaleLinear().domain([0, Math.PI, 2 * Math.PI]).range(["red", "blue", "red"]); // #9d174d

    const springProps = useSpring({
            from: {
                value: 0,
                textPos: -10,
                arg: 0
            },
            to: {
                value: value,
                textPos: isNegative ? +10 : -10,
                arg: comp.arg()
            }
    });

    console.log(barWidth, barWidth < 40 ? 90 : 0);

    return (
        <g>
            <animated.rect
                x={x}
                y={springProps.value?.to(value => value < 0 ? yScale(0) : yScale(value))}
                width={barWidth}
                height={springProps.value?.to(value => Math.abs(yScale(value) - yScale(0)))}
                opacity={0.7}
                stroke={springProps.arg?.to(color => colorScale(color))}
                fill={springProps.arg?.to(color => colorScale(color))}
                fillOpacity={0.7}
                strokeWidth={1}
                rx={1}
            />
            <animated.text
                x={x + xScale.bandwidth() / 2}
                y={springProps.textPos?.to(textPos => yScale(0) + textPos)}
                textAnchor="middle"
                alignmentBaseline="central"
                fontSize={12}
            >
                {springProps.value?.to(value => (value * 100).toFixed(2) + "%")}
            </animated.text>
            <animated.text
                x={x + xScale.bandwidth() / 2}
                y={springProps.textPos?.to(textPos => yScale(0) - textPos)}
                textAnchor="middle"
                alignmentBaseline="central"
                fontSize={12}
            >
                {res}
            </animated.text>
        </g>
    );
}

const MARGIN = { top: 30, right: 30, bottom: 30, left: 30 };
const BAR_PADDING = 0.3;

function BarChart ({ components, width, height, ref }) {
    const boundsWidth = width - MARGIN.right - MARGIN.left;
    const boundsHeight = height - MARGIN.top - MARGIN.bottom;

    const groups = components.sort((a, b) => a.res - b.res).map(d => d.res);
    const xScale = useMemo(() => {
        return d3
            .scaleBand()
            .domain(groups)
            .range([0, boundsWidth])
            .padding(BAR_PADDING);
    }, [components, width]);

    // Y axis
    const yScale = d3.scaleLinear().domain([1, -1]).range([0, boundsHeight]);

    const grid = yScale
        .ticks(5)
        .map((value, i) => (
            <g key={i}>
                <line
                    x1={0}
                    x2={boundsWidth}
                    y1={yScale(value)}
                    y2={yScale(value)}
                    stroke="#808080"
                    opacity={0.2}
                />
                <text
                    x={-10}
                    y={yScale(value)}
                    textAnchor="middle"
                    alignmentBaseline="central"
                    fontSize={9}
                    fill="#808080"
                    opacity={0.8}
                >
                    {value * 100}%
                </text>
            </g>
    ));

    const shapes = components.map(d => <ProbabilityItem
        key={d.res}
        xScale={xScale}
        yScale={yScale}
        boundsHeight={boundsHeight}
        {...d} />
    );

    return <div ref={ref} style={{ height: '100%', width: '100%' }}>
        <svg width={width} height={height}>
            <g
            width={boundsWidth}
            height={boundsHeight}
            transform={`translate(${[MARGIN.left, MARGIN.top].join(",")})`}
            >
                {grid}
                {shapes}
            </g>
        </svg>
    </div>
}

export default function Probability ({ components }) {
    const ref = useRef(null);
    const { width, height } = useDimensions(ref);
    return <BarChart ref={ref} width={width} height={height} components={components} />
}
