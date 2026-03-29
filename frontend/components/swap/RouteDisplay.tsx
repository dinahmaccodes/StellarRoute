'use client';

import { Skeleton } from "@/components/ui/skeleton";
import { ArrowRight, Info, ChevronDown, MapPin } from "lucide-react";
import { useState, useMemo } from "react";
import { Badge } from "@/components/ui/badge";
import { ConfidenceIndicator } from "./ConfidenceIndicator";
import { RouteDisplaySkeleton } from "./RouteDisplaySkeleton";
import { cn } from "@/lib/utils";

interface RouteDisplayProps {
  route: string[];
  amountOut: string;
  confidenceScore?: number;
  volatility?: "high" | "medium" | "low";
  isLoading?: boolean;
}

export function RouteDisplay({
  route,
  amountOut,
  confidenceScore = 85,
  volatility = "low",
  isLoading = false,
}: RouteDisplayProps) {
  const [showDetails, setShowDetails] = useState(false);

  if (isLoading) {
    return <RouteDisplaySkeleton />;
  }

  if (!route || route.length < 2) {
    return null;
  }

  return (
    <div className="rounded-2xl border border-border/40 bg-background/40 backdrop-blur-sm p-4 space-y-4 transition-all duration-300 hover:border-primary/20 hover:shadow-md group">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2">
          <div className="p-1.5 rounded-lg bg-primary/10 text-primary">
            <MapPin className="h-3.5 w-3.5" />
          </div>
          <h4 className="text-sm font-semibold tracking-tight text-foreground/90">Optimal Route</h4>
        </div>
        <div className="flex items-center gap-2">
          <ConfidenceIndicator score={confidenceScore} volatility={volatility} />
          <Badge variant="secondary" className="text-[10px] font-bold uppercase tracking-wider bg-emerald-500/10 text-emerald-600 hover:bg-emerald-500/20 border-emerald-500/20 px-1.5 h-5 transition-colors">
            Best Price
          </Badge>
          <button
            type="button"
            onClick={() => setShowDetails((prev) => !prev)}
            aria-expanded={showDetails}
            className="h-8 w-8 flex items-center justify-center rounded-full hover:bg-muted/80 transition-all active:scale-90"
          >
            <ChevronDown
              className={cn(
                "h-4 w-4 text-muted-foreground transition-transform duration-300",
                showDetails ? "rotate-180" : ""
              )}
            />
          </button>
        </div>
      </div>

      <div className="relative flex items-center justify-between px-2 py-3 bg-muted/30 rounded-xl border border-border/20">
        {route.map((token, index) => (
          <div key={`route-${index}-${token}`} className="flex items-center gap-2 flex-1 justify-center first:justify-start last:justify-end">
            <div className="flex flex-col items-center sm:items-start">
              <span className="text-xs font-bold text-foreground">{token}</span>
              {index === route.length - 1 && (
                <span className="text-[10px] text-muted-foreground font-medium truncate max-w-[70px]">
                  {parseFloat(amountOut).toFixed(4)}
                </span>
              )}
            </div>
            {index < route.length - 1 && (
              <div className="flex items-center justify-center flex-1 min-w-[30px] px-1">
                <div className="h-[1px] flex-1 bg-border/60 relative">
                  <ArrowRight className="h-3 w-3 text-muted-foreground absolute left-1/2 -translate-x-1/2 -translate-y-1/2 bg-transparent" />
                </div>
              </div>
            )}
          </div>
        ))}
      </div>

      {showDetails && (
        <div className="pt-3 border-t border-border/20 animate-in fade-in slide-in-from-top-1 duration-200">
          <div className="space-y-2">
            <div className="flex items-center justify-between text-[11px] text-muted-foreground font-medium uppercase tracking-wider">
              <span>Hops</span>
              <span>Provider</span>
            </div>
            {route.length > 2 ? (
              <div className="space-y-1.5">
                {route.slice(0, -1).map((token, i) => (
                  <div key={`hop-${i}`} className="flex items-center justify-between p-2 rounded-lg bg-background/50 border border-border/10 text-xs">
                    <div className="flex items-center gap-2">
                      <span className="font-semibold">{token}</span>
                      <ArrowRight className="h-3 w-3 text-muted-foreground" />
                      <span className="font-semibold">{route[i+1]}</span>
                    </div>
                    <span className="text-[10px] font-medium px-1.5 py-0.5 rounded-md bg-muted text-muted-foreground">
                      {i % 2 === 0 ? 'AQUA Pool' : 'Orderbook'}
                    </span>
                  </div>
                ))}
              </div>
            ) : (
              <div className="flex items-center justify-between p-2 rounded-lg bg-background/50 border border-border/10 text-xs">
                <div className="flex items-center gap-2">
                  <span className="font-semibold">{route[0]}</span>
                  <ArrowRight className="h-3 w-3 text-muted-foreground" />
                  <span className="font-semibold">{route[1]}</span>
                </div>
                <span className="text-[10px] font-medium px-1.5 py-0.5 rounded-md bg-muted text-muted-foreground">
                  Direct Path
                </span>
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
