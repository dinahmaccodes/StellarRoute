import { render, screen } from "@testing-library/react";
import { cleanup } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { RouteDisplay } from "./RouteDisplay";

describe("RouteDisplay", () => {
  afterEach(() => cleanup());

  it("should render loading skeleton when isLoading is true", () => {
    render(
      <RouteDisplay isLoading={true} route={[]} amountOut="0.0" />
    );

    // Check for skeleton elements (animate-pulse class)
    const skeletonElements = document.querySelectorAll(".animate-pulse");
    expect(skeletonElements.length).toBeGreaterThanOrEqual(1);
  });

  it("should render actual content when isLoading is false or undefined", () => {
    const mockRoute = ['XLM', 'USDC'];

    render(
      <RouteDisplay isLoading={false} route={mockRoute} amountOut="50.0" />
    );

    expect(screen.getByText(/optimal route/i)).toBeInTheDocument();
  });

  it("should accept isLoading prop as true", () => {
    const { container } = render(
      <RouteDisplay isLoading={true} route={[]} amountOut="0.0" />
    );

    // Verify skeleton is rendered by checking for skeleton elements
    const skeletons = container.querySelectorAll(".animate-pulse");
    expect(skeletons.length).toBeGreaterThan(0);
  });

  it("should accept isLoading prop as false", () => {
    const { container } = render(
      <RouteDisplay isLoading={false} route={[]} amountOut="0.0" />
    );

    // Verify content is rendered (not skeleton)
    const skeletons = container.querySelectorAll(".animate-pulse");
    expect(skeletons.length).toBe(0);
  });
});
