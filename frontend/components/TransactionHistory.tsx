"use client"

import { useState, useEffect, useRef } from "react"
import { useVirtualizer } from "@tanstack/react-virtual"
import { useTransactionHistory } from "@/hooks/useTransactionHistory"
import { TransactionRecord } from "@/types/transaction"
import { 
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Card } from "@/components/ui/card"
import { ScrollArea } from "@/components/ui/scroll-area"
import { ActivityTableSkeleton } from "@/components/shared/ActivityTableSkeleton"
import { formatDistanceToNow } from "date-fns"
import { ExternalLink, Trash2, ArrowRight } from "lucide-react"
import { CopyButton } from "@/components/shared/CopyButton"

// Hardcode mock wallet to match DemoSwap
const MOCK_WALLET = "GBSU...XYZ9"

// VIRTUALIZATION_THRESHOLD: The number of items beyond which the transaction list is virtualized.
const VIRTUALIZATION_THRESHOLD = 50;

export function TransactionHistory() {
  const { transactions, clearHistory } = useTransactionHistory(MOCK_WALLET)
  const [filterAsset, setFilterAsset] = useState<string>("ALL")
  const [sortKey, setSortKey] = useState<"date" | "amount">("date")
  const [isLoading, setIsLoading] = useState(true)

  // Simulate loading for 300ms to show skeleton and prevent flicker on fast data
  useEffect(() => {
    const timer = setTimeout(() => {
      setIsLoading(false)
    }, 300)
    return () => clearTimeout(timer)
  }, [])

  const scrollRef = useRef<HTMLDivElement>(null)

  // Filtering and Sorting
  const filteredTxs = transactions.filter((tx) => {
    if (filterAsset === "ALL") return true
    return tx.fromAsset === filterAsset || tx.toAsset === filterAsset
  })

  const sortedTxs = [...filteredTxs].sort((a, b) => {
    if (sortKey === "date") {
      return b.timestamp - a.timestamp
    } else {
      return parseFloat(b.fromAmount) - parseFloat(a.fromAmount)
    }
  })

  const getStatusBadge = (status: TransactionRecord["status"]) => {
    switch (status) {
      case "success":
        return <Badge className="bg-success">Success</Badge>
      case "failed":
        return <Badge variant="destructive">Failed</Badge>
      case "pending":
      case "submitting":
      case "processing":
        return <Badge variant="secondary">Processing</Badge>
      default:
        return <Badge variant="outline">{status}</Badge>
    }
  }

  const isVirtualized = sortedTxs.length >= VIRTUALIZATION_THRESHOLD;
  const rowVirtualizer = useVirtualizer({
    count: sortedTxs.length,
    getScrollElement: () => scrollRef.current,
    estimateSize: () => 64, // Estimated row height
    overscan: 5,
  })

  return (
    <Card className="flex flex-col h-[calc(100vh-140px)] m-4 shadow-sm border-primary/10">
      <div className="p-4 border-b flex flex-col sm:flex-row justify-between items-center gap-4 bg-muted/30">
        <div>
          <h2 className="text-2xl font-bold">Transaction History</h2>
          <p className="text-sm text-muted-foreground mt-1 flex items-center gap-1">
            Wallet:{" "}
            <span className="font-mono text-foreground">{MOCK_WALLET}</span>
            <CopyButton value={MOCK_WALLET} label="Copy wallet address" />
          </p>
        </div>

        <div className="flex items-center gap-2">
           <select 
              className="h-9 px-3 text-sm border rounded-md bg-background focus:outline-none focus:ring-1 focus:ring-ring"
              value={filterAsset} 
              onChange={(e) => setFilterAsset(e.target.value)}
            >
              <option value="ALL">All Tokens</option>
              <option value="XLM">XLM</option>
              <option value="USDC">USDC</option>
           </select>
           
           <select 
              className="h-9 px-3 text-sm border rounded-md bg-background focus:outline-none focus:ring-1 focus:ring-ring"
              value={sortKey} 
              onChange={(e) => setSortKey(e.target.value as "date" | "amount")}
            >
              <option value="date">Sort by Date</option>
              <option value="amount">Sort by Amount</option>
           </select>

           <Button variant="outline" size="icon" onClick={clearHistory} title="Clear History">
              <Trash2 className="h-4 w-4 text-destructive" />
           </Button>
        </div>
      </div>

      <ScrollArea className="flex-1" viewportRef={scrollRef}>
        {isLoading ? (
          <ActivityTableSkeleton />
        ) : sortedTxs.length === 0 ? (
          <div className="flex flex-col items-center justify-center p-12 text-center h-full">
            <div className="text-muted-foreground w-16 h-16 mb-4 opacity-50 bg-muted rounded-full flex items-center justify-center">
               <span className="text-2xl">📋</span>
            </div>
            <h3 className="text-xl font-semibold mb-1">No Transactions Found</h3>
            <p className="text-sm text-muted-foreground max-w-[250px]">
              You haven&apos;t made any swaps yet, or your filters are too restrictive.
            </p>
          </div>
        ) : (
          <Table>
            <TableHeader className="bg-muted/50 sticky top-0 z-10">
              <TableRow className="flex w-full">
                <TableHead className="w-[180px] shrink-0">Date</TableHead>
                <TableHead className="flex-1 min-w-[200px]">Swap</TableHead>
                <TableHead className="w-[180px] shrink-0">Rate</TableHead>
                <TableHead className="w-[120px] shrink-0">Status</TableHead>
                <TableHead className="w-[100px] shrink-0 text-right">Explorer</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody 
              className="block"
              style={{
                height: isVirtualized ? `${rowVirtualizer.getTotalSize()}px` : 'auto',
                position: isVirtualized ? 'relative' : 'static',
              }}
            >
              {(isVirtualized ? rowVirtualizer.getVirtualItems() : sortedTxs.map((tx, index) => ({ index } as unknown as ReturnType<typeof rowVirtualizer.getVirtualItems>[0]))).map((virtualRow) => {
                const tx = sortedTxs[virtualRow.index]
                return (
                 <TableRow 
                   key={tx.id}
                   data-index={virtualRow.index}
                   ref={isVirtualized ? rowVirtualizer.measureElement : null}
                   className="flex w-full"
                   style={isVirtualized ? {
                     position: 'absolute',
                     top: 0,
                     left: 0,
                     width: '100%',
                     transform: `translateY(${virtualRow.start}px)`
                   } : undefined}
                 >
                   <TableCell className="font-medium w-[180px] shrink-0">
                    <div className="flex flex-col">
                      <span>{new Date(tx.timestamp).toLocaleDateString()}</span>
                      <span className="text-xs text-muted-foreground whitespace-nowrap">
                        {formatDistanceToNow(tx.timestamp, { addSuffix: true })}
                      </span>
                    </div>
                  </TableCell>
                  <TableCell className="flex-1 min-w-[200px]">
                    <div className="flex items-center gap-3">
                       <div className="flex flex-col">
                          <span className="font-bold text-sm">-{tx.fromAmount}</span>
                          <span className="text-xs text-muted-foreground">{tx.fromAsset}</span>
                       </div>
                       <ArrowRight className="w-4 h-4 text-muted-foreground/50" />
                       <div className="flex flex-col">
                          <span className="font-bold text-sm text-success">+{tx.toAmount}</span>
                          <span className="text-xs text-muted-foreground">{tx.toAsset}</span>
                       </div>
                    </div>
                  </TableCell>
                  <TableCell className="text-xs text-muted-foreground w-[180px] shrink-0">
                    1 {tx.fromAsset} = {tx.exchangeRate} {tx.toAsset}
                  </TableCell>
                  <TableCell className="w-[120px] shrink-0">
                    {getStatusBadge(tx.status)}
                    {tx.errorMessage && (
                      <div className="text-[10px] text-destructive mt-1 max-w-[150px] truncate" title={tx.errorMessage}>
                        {tx.errorMessage}
                      </div>
                    )}
                  </TableCell>
                  <TableCell className="text-right w-[100px] shrink-0">
                    {tx.hash ? (
                      <div className="inline-flex items-center gap-1 justify-end">
                        <CopyButton value={tx.hash} label="Copy transaction hash" />
                        <a
                          href={`https://stellar.expert/explorer/public/tx/${tx.hash}`}
                          target="_blank"
                          rel="noreferrer"
                          className="inline-flex items-center gap-1 text-xs text-primary hover:underline group"
                        >
                          <span className="hidden sm:inline">View</span>
                          <ExternalLink className="w-3 h-3 group-hover:translate-x-px group-hover:-translate-y-px transition-transform" />
                        </a>
                      </div>
                    ) : (
                      <span className="text-xs text-muted-foreground">-</span>
                    )}
                  </TableCell>
                </TableRow>
              )})}
            </TableBody>
          </Table>
        )}
      </ScrollArea>
    </Card>
  )
}
