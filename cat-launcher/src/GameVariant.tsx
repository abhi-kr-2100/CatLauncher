import { GameVariantInfo } from "./utils";

export interface GameVariantProps {
    variant: GameVariantInfo;
}

export default function GameVariant(props: GameVariantProps) {
  const { variant } = props;
  
  return (
    <div>
      <h2>{variant.name}</h2>
      <p>{variant.description}</p>
    </div>
  );
}
