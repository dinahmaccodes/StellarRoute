import { act, cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { SettingsProvider } from "@/components/providers/settings-provider";
import { SwapCard } from "./SwapCard";

vi.mock("next-themes", () => ({
  useTheme: () => ({
    theme: "light",
    setTheme: vi.fn(),
  }),
}));

function renderSwapCard() {
  return render(
    <SettingsProvider>
      <SwapCard />
    </SettingsProvider>,
  );
}

function setNavigatorOnline(value: boolean) {
  Object.defineProperty(window.navigator, "onLine", {
    configurable: true,
    value,
  });
}

describe("SwapCard network resilience", () => {
  beforeEach(() => {
    localStorage.clear();
  });

  afterEach(() => {
    cleanup();
    vi.useRealTimers();
  });

  it("shows offline state clearly and blocks submission while disconnected", async () => {
    setNavigatorOnline(false);
    renderSwapCard();

    await screen.findByText(/you're offline/i);

    fireEvent.change(screen.getByLabelText("Pay amount"), {
      target: { value: "10" },
    });

    expect(
      screen.getByText("You are offline. Reconnect to refresh quote."),
    ).toBeInTheDocument();
    expect(screen.getByRole("button", { name: "Retry quote" })).toBeInTheDocument();

    const cta = screen.getByRole("button", { name: "Offline" });
    expect(cta).toBeDisabled();
  });

  it("automatically recovers quotes after reconnecting", async () => {
    setNavigatorOnline(false);
    renderSwapCard();

    await screen.findByLabelText("Pay amount");
    fireEvent.change(screen.getByLabelText("Pay amount"), {
      target: { value: "10" },
    });

    expect(
      screen.getByText("You are offline. Reconnect to refresh quote."),
    ).toBeInTheDocument();

    act(() => {
      setNavigatorOnline(true);
      window.dispatchEvent(new Event("online"));
    });

    await waitFor(
      () => {
        expect(
          screen.queryByText("You are offline. Reconnect to refresh quote."),
        ).not.toBeInTheDocument();
        expect(screen.getByLabelText("Receive amount")).toHaveValue("9.8000");
        expect(screen.getByRole("button", { name: "Review Swap" })).toBeEnabled();
      },
      { timeout: 2000 },
    );
  });
});
