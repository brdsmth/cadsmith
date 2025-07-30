# Geometry Kernel Development Plan

This document outlines the remaining major features to be implemented in the geometry kernel, based on the initial requirements.

## 1. Modify Geometry

-   **Fillets**: Implement functionality to create fillets (rounded edges) on existing geometry.

## 2. Boolean Operations

-   **Union**: Combine two solids into a single solid.
-   **Difference**: Subtract one solid from another.
-   **Intersection**: Find the common volume between two solids.

## 3. Evaluate Geometry

-   **Intersections**: Develop methods to calculate intersection points/lines/faces between geometric entities (e.g., line-plane, plane-solid).
-   **Normals**: Ensure robust calculation and retrieval of surface normals for all geometric entities.
-   **Point-Inside Test**: Implement a function to determine if a given point lies inside or outside a solid.

## Future Considerations

-   **3D GUI Viewer**: Develop a simple 3D GUI viewer using `egui` and `wgpu` to visualize and verify the geometry kernel's output. This will be crucial for debugging and demonstrating capabilities.
