import {
  PieChart,
  Pie,
  Cell,
  Tooltip,
  ResponsiveContainer,
} from "recharts";

import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import { usePlayTimeData } from "../hooks/usePlayTimeData";

interface PlayTimeChartProps {
  variants: GameVariantInfo[];
}

interface PieDataItem {
  name: string;
  playTime: number;
  [key: string]: string | number;
}

const COLORS = [
  "#8884d8",
  "#82ca9d",
  "#ffc658",
  "#ff7c7c",
  "#8dd1e1",
  "#d084d0",
  "#a4de6c",
  "#ffa58f",
];

interface PlayTimePieTooltipProps {
  active?: boolean;
  payload?: Array<{
    name: string;
    value: number;
    payload: { name: string };
  }>;
  totalTime: number;
}

function PlayTimePieTooltip({
  active,
  payload,
  totalTime,
}: PlayTimePieTooltipProps) {
  if (active && payload && payload.length) {
    const entry = payload[0];
    const percentage = (
      (totalTime === 0 ? 0 : entry.value / totalTime) * 100
    ).toFixed(1);
    return (
      <div className="bg-popover border border-border rounded px-3 py-2 text-sm z-50 relative">
        <p className="font-semibold">{entry.payload.name}</p>
        <p className="text-muted-foreground">
          {entry.value.toFixed(1)} hours ({percentage}%)
        </p>
      </div>
    );
  }
  return null;
}

export default function PlayTimeChart({
  variants,
}: PlayTimeChartProps) {
  const { playTimeData, isLoading } = usePlayTimeData(variants);

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <p className="text-muted-foreground">
          Loading play time data...
        </p>
      </div>
    );
  }

  if (!playTimeData || playTimeData.length === 0) {
    return (
      <div className="flex items-center justify-center h-64">
        <p className="text-muted-foreground">
          No play time data available
        </p>
      </div>
    );
  }

  const totalTime = playTimeData.reduce(
    (sum, item) => sum + item.playTime,
    0,
  );

  const calculatePercentage = (value: number): string => {
    return ((totalTime === 0 ? 0 : value / totalTime) * 100).toFixed(
      1,
    );
  };

  return (
    <div className="flex flex-col items-center w-full">
      <div className="w-full h-96 flex items-center justify-center relative">
        <div className="absolute flex flex-col items-center justify-center pointer-events-none">
          <p className="text-2xl font-semibold">
            {totalTime.toFixed(1)}
          </p>
          <p className="text-sm text-muted-foreground">hours</p>
        </div>
        <ResponsiveContainer width="100%" height="100%">
          <PieChart>
            <Pie
              data={playTimeData as PieDataItem[]}
              cx="50%"
              cy="50%"
              labelLine={false}
              label={({ value }) => {
                const percentage = calculatePercentage(
                  value as number,
                );
                return `${percentage}%`;
              }}
              outerRadius={100}
              innerRadius={60}
              fill="#8884d8"
              dataKey="playTime"
            >
              {playTimeData.map((_, index) => (
                <Cell
                  key={`cell-${index}`}
                  fill={COLORS[index % COLORS.length]}
                />
              ))}
            </Pie>
            <Tooltip
              content={<PlayTimePieTooltip totalTime={totalTime} />}
            />
          </PieChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
}
