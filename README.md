# EGA

This project is a work-in-progress implementation of a PGA library for 3D
computer graphics.

---

This library is an implementation of an R{3,0,1} geometric algebra, which is
the smallest geometric algebra that can encode 3D euclidian geometry.

Other algebras in the same family include the R{2,0,1} 2D PGA, and the R{4,1}
Conformal Geometric Algebra. The CGA can also encode euclidian geometry (among
other things like spheres), however its full multivector has 25 elements. This
means a full geometric product is quite expensive to compute in CGA.

Compared to CGA (R{4,1}), the R{3,0,1} PGA (this library), has a multivector
with only 16 elements and a computational cost roughly equivalent to a 4x4
matrix in linear algebra.

Furthermore, this library uses the type system along with the product rules of
the algebra to reduce the computational & memory costs where possible. For
instance we store only the vector components `e0`,`e1`,`e2`,`e3` as a `Vector`
and we know that the outer-product of a vector with another vector is a
bivector (on a 6 element basis). so `Vector.meet(Vector) -> Bivector`. This
helps to avoid full-16 element products in many cases.

See [bivector.net](https://bivector.net/PGA4CS.html) for more information
about the mathematics.
